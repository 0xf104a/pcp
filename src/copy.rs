use std::process::exit;
use colored::Colorize;
use crate::progress::ProgressDisplay;
use crate::reader::Reader;
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

pub async fn copy(mut reader: Box<dyn Reader>, mut writer: Box<dyn Writer>,
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