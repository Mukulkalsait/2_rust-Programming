import os
import signal
import time
import select
import socket

r, w = os.pipe()
pid = os.fork()


# ---------------------------------
# if pid != 0:
#     print("Paretnt Process")
#     print("Parent PID :", os.getpid())
#     print("Child PID :", pid)
# else: 
#     print("Child Process")
#     print("Child PID :", os.getpid())
#     print("parent PID :", os.getppid())

# ---------------------------------
# if pid == 0: 
#     print("Child : EXICUTE 'pwd' ")
#     os.execvp("pwd",["pwd"])
# else:
#     os.wait()
#     print("Child Died-> Parent get sigchld and died.")

# ---------------------------------
# if pid == 0:
#     print("Exit Child:")
#     os._exit(0)
# else:
#     child_pid, status = os.wait()
#     print("Status: ", status)
#     print("Parent: Child '", child_pid, "' Finished")

# B:IPC = Inter Process Communications

# if pid == 0:
#     os.close(w)
#     msg = os.read(r, 100)
#     print("child received:", msg,decode())
# else:
#     os.close(r)
#     os.write(w, b"Hello from paretn")
#     os.wait()

# B: CREATE zombie Process
#
# def handler(signum, frame):
#     print("Recieved signal:", signum)
#
# signal.signal(signal.SIGINT, handler)
#
# print("stop with Ctro + c")
#
# while True:
#     time.sleep(1)

# B: FILE MANAGEEMENT
#
# fd = os.open("text.txt", os.O_CREAT | os.O_WRONLY)
# os.write(fd, b"hello")
# os.close(fd)

def run_epoll_server():
    server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    server.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1) # allow socker reuse

    server.bind(("127.0.0.1", 9090))
    server.listen()

    epoll = select.epoll()
    epoll.register(server.fileno(),select.EPOLLIN)

    print("Server Runnign on port localhost:9090")

    try: 
        while True: 
            events = epoll.poll(1)
            for fileno, event in events:
                if fileno ==  server.fileno():
                    conn, addr = server.accept()
                    print("connection  from", addr)
                    conn.close()
    finally:
        epoll.unregister(server.fileno())
        epoll.close()
        server.close()


if __name__  == "__main__":
    run_epoll_server()
























