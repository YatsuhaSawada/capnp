fn main() -> Result<(), Box<dyn std::error::Error>> {
    capnpc::CompilerCommand::new()
        .file("hello_world.capnp")
        .file("echo.capnp")
        .run()?;
    Ok(())
}
