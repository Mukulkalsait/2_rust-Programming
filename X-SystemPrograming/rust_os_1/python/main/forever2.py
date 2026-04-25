import os
import sys
import subprocess
import signal

if len(sys.argv) < 2 :
    print("How to use: \n python3 forever.py <command> [args]")
    sys.exit(1)

cmd = sys.argv[1:]
file_op= open("output.logs","a")
file_err= open("error.logs","a")
file_reg= open("registry.txt","a")
is_terminated = False
signal_no = 0

try: 
    bg_process = subprocess.Popen( cmd, stdin=subprocess.DEVNULL, stdout=file_op, stderr=file_err,)
    print(f"Parent '{os.getpid()}' started Background process \n|=> '{bg_process.pid}'")

except Exception as e:
    print(f"Failed to run run background Process {e}")
    file_op.close()
    file_err.close()
    file_reg.close()
    sys.exit(1)

def kill_process(signum, current_stack_p):
    global is_terminated, signal_no
    is_terminated = True
    signal_no = signum
    print(f" \nIntrruption Signal {signum} Recieved, Trying to stop the Background Process {bg_process.pid}")
    bg_process.terminate()
    print(f"{bg_process.pid} Gracefully Stoped.")

signal.signal(signal.SIGTERM, kill_process)
signal.signal(signal.SIGINT, kill_process)

try:
    if is_terminated: 
        try:
            bg_process.wait(timeout=3)
            print(f"Background process Gracefully Stoped.")
        except:
            print("Failed Graceful Termination: Trying To Force Kill")
            bg_process.kill()
    else:
        bg_process.wait()

except IntrruptionError:
    pass

finally:
    file_op.close()
    file_err.close()
    file_reg.close()

if not is_terminated:
    print("Process Died Naturally.")
