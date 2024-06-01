# a tcp client connect to 0.0.0.0:9302 and send some data
# the server will receive the data and print it out
import socket
import time

socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
socket.connect(("0.0.0.0", 9302))
socket.send(b"hello")
print(socket.recv(1024))
socket.close()
