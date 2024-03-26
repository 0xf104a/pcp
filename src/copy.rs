use colored::Colorize;

use crate::arguments::Args;
use crate::factories::{get_reader_proxy_for_url, get_writer_proxy_for_url};
use crate::progress::console::ConsoleProgress;
use crate::progress::dummy::DummyProgress;
use crate::progress::ProgressDisplay;
use crate::reader::Reader;
use crate::utils::runtime::tokio_block_on;
use crate::writer::Writer;

pub type DynBuffer = Vec<u8>;

trait Buffer{
    fn make_buffer(size: usize) -> Self where Self: Sized;
}

impl Buffer for DynBuffer{
    fn make_buffer(size: usize) -> DynBuffer {
        let mut buffer = DynBuffer::with_capacity(size);
        for _ in 0..size{
            buffer.push(0);
        }
        buffer
    }
}
#[inline]
fn handle_error_if_needed(result: std::io::Result<usize>) -> bool{
    if result.is_err(){
        let error = result.err().unwrap();
        crate::utils::term::flush();
        println!("{}{}: {}", "Can not write destination".bold().red(), "".clear(),
                 error.to_string());
        true
    } else { 
        false
    }
}

fn get_progress_from_args(args: &Args) -> Box<dyn ProgressDisplay>{
    if args.no_progress{
        Box::new(DummyProgress::new())
    } else {
        Box::new(ConsoleProgress::new())
    }
}

async fn do_copy(mut reader: Box<dyn Reader>, mut writer: Box<dyn Writer>,
                       mut progress: Box<dyn ProgressDisplay>,
                       max_chunks_staged: usize,
                       chunk_size: usize) -> bool{
    let (tx, mut rx) =
        tokio::sync::mpsc::channel::<Option<(usize, DynBuffer)>>(max_chunks_staged);
    let size = reader.get_size();
    progress.set_size(size);
    let read_coroutine = async move{
        let mut buffer = DynBuffer::make_buffer(chunk_size);
        loop {
            let bytes_read = reader.read_chunk(&mut buffer, chunk_size).await;
            //println!("{}", bytes_read);
            if bytes_read == 0{
                tx.send(None).await.expect("Can not send buffer");
                break;
            }
            if tx.send(Some((bytes_read, buffer.clone()))).await.is_err(){
                break;
            }
        }
        true
    };
    let write_coroutine = async move {
        let mut result = true;
        loop  {
            let chunk_wrapped = rx.recv().await.unwrap();
            if chunk_wrapped.is_none(){
                break;
            }
            let (n, chunk) = chunk_wrapped.unwrap();
            if handle_error_if_needed(writer.write_chunk(&chunk, n).await){
                result = false;
                break;
            }
            progress.add_bytes_written(n);
        }
        progress.flush();
        result
    };
    let (result_read, result_write) = tokio::join!(read_coroutine, write_coroutine);
    result_read && result_write
}

pub fn copy_file(source: &str, target: &str, args: &Args) -> bool{
    let writer_proxy = get_writer_proxy_for_url(target).unwrap();
    let reader_proxy = get_reader_proxy_for_url(source).unwrap();
    let mut progress = get_progress_from_args(args);
    let mut str_target = target.to_string();
    if writer_proxy.is_directory(target){
        let filename = reader_proxy.filename(source);
        str_target = writer_proxy.join_path(target, &filename);
    }
    let reader = reader_proxy.produce(source);
    let writer = writer_proxy.produce(&str_target);
    progress.set_progress(&*format!("{} -> {}", source, target), 0);
    let buffer_size = reader.get_blocksize();
    tokio_block_on(do_copy(reader, writer, progress,
                           args.max_chunks_number as usize, buffer_size))

}

pub fn copy_directory(source: &str, target: &str, args: &Args) -> bool{
    let mut target_path = target.to_string();
    let writer_proxy = get_writer_proxy_for_url(target).unwrap();
    let reader_proxy = get_reader_proxy_for_url(source).unwrap();
    let is_new_dir = if writer_proxy.is_directory(target){
        target_path = writer_proxy.join_path(&target_path, &reader_proxy.dirname(source));
        false
    } else {
        writer_proxy.make_directory(target);
        true
    };
    for object in reader_proxy.iter_directory(source){
        //println!("{}", object);
        let target_object = if is_new_dir{
            reader_proxy.relative_path(&reader_proxy.dirname(&source), &object)
        } else {
            object.clone()
        };
        let reader = reader_proxy.produce(&object);
        let destination = writer_proxy.join_path(&target_path, &target_object);
        //println!("target_path={}, object={}, dest={}", target_path, target_object, destination);
        let mut progress = get_progress_from_args(args);
        let buffer_size = reader.get_blocksize();
        if reader_proxy.is_directory(&object){
            writer_proxy.make_directory(&destination);
            continue;
        }
        let writer = writer_proxy.produce(&destination);
        progress.set_progress(&*format!("{} -> {}", object, destination), 0);
        let coroutine = async move {
            do_copy(reader, writer, progress, args.max_chunks_number as usize, buffer_size).await
        };
        if !tokio_block_on(coroutine){
            return false;
        }
    }
    true
}