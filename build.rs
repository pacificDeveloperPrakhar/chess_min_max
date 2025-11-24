pub fn main()->()
{
    let proto_file = "./chess.proto";
    tonic_prost_build::configure()
        .build_server(true)
        .out_dir("./src")
        .compile_protos(&[proto_file], &["."]).expect("unable to compile the file");
}