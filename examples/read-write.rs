use tokio::{
    fs::File,
    io::{self, AsyncReadExt, AsyncWriteExt},
};

#[tokio::main]
async fn main() -> io::Result<()> {
    write().await?;
    write_all().await?;
    read().await?;
    read_to_end().await?;
    copy().await?;
    Ok(())
}

async fn read() -> io::Result<()> {
    let mut f = File::open("foo.txt").await?;
    let mut buffer = [0; 10];
    let n = f.read(&mut buffer[..]).await?;
    println!("The bytes: {:?}", &buffer[..n]);
    Ok(())
}

async fn read_to_end() -> io::Result<()> {
    let mut f = File::open("foo.txt").await?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).await?;
    println!("The bytes: {:?}", buffer);
    Ok(())
}

async fn write() -> io::Result<()> {
    let mut f = File::create("foo.txt").await?;
    let n = f.write(b"some bytes").await?;
    println!("Wrote the first {} bytes of 'some bytes'.", n);
    Ok(())
}

async fn write_all() -> io::Result<()> {
    let mut f = File::create("foo.txt").await?;
    f.write_all(b"some bytes write all").await?;
    Ok(())
}

async fn copy() -> io::Result<()> {
    let mut reader: &[u8] = b"hello";
    let mut f = File::create("foo.txt").await?;
    io::copy(&mut reader, &mut f).await?;
    Ok(())
}
