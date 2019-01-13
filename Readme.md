cargo +nighly run

Windows Installation:
zeromq : http://zeromq.org/distro:microsoft-windows
openssl binaries : https://wiki.openssl.org/index.php/Binaries https://github.com/sfackler/rust-openssl#windows


Place zmq.lib in C:\Program Files\ZeroMQ 4.0.4\lib
ZMQ VARIABLES
$Env:LIBZMQ_LIB_DIR="C:\Program Files\ZeroMQ 4.0.4\lib"
$Env:LIBZMQ_INCLUDE_DIR="C:\Program Files\ZeroMQ 4.0.4\include"

OPENSSL VARIABLES
$Env:OPENSSL_DIR="C:\OpenSSL-Win64"