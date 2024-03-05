use crate::progress::ProgressDisplay;
use crate::reader::Reader;
use crate::writer::Writer;

pub type DynBuffer = Vec<u8>;

pub async fn copy(reader: Box<dyn Reader>, mut writer: Box<dyn Writer>,
            mut progress: Box<dyn ProgressDisplay>,
            max_chunks_staged: usize,
            chunk_size: usize){
    let (tx, mut rx) =
        tokio::sync::mpsc::channel::<Option<(usize, DynBuffer)>>(max_chunks_staged);
    let size = reader.get_size();
    progress.set_size(size);
    let read_coroutine = async move{
        let mut buffer = DynBuffer::with_capacity(chunk_size);
        loop {
            let bytes_read = reader.read_chunk(&mut buffer, chunk_size).await;
            if bytes_read == 0{
                tx.send(None).await.expect("Can not send buffer");
                break;
            }
            tx.send(Some((bytes_read, buffer.clone()))).await.expect("Can not send buffer");
        }
    };
    let write_coroutine = async move {
        loop  {
            let chunk_wrapped = rx.recv().await.unwrap();
            if chunk_wrapped.is_none(){
                break;
            }
            let (n, chunk) = chunk_wrapped.unwrap();
            writer.write_chunk(&chunk).await;
            progress.add_bytes_written(n);
        }
    };
    let _ = tokio::join!(read_coroutine, write_coroutine);
}