import os 
import sys
import subprocess
import signal
import time


#----------------------------------------------------
# Y: arg handler
if len(sys.argv) < 2:
    print("Usage: python3 problem_1.py <command> [args...]")
    sys.exit(1)

#----------------------------------------------------
# B: DECLERATIONS

stdout_log = open("out.log","a")
stderr_log = open("err.log","a")

command = sys.argv[1:]
child_proc = None
max_restart_tries = 3
running_status = True
restart_count = 0

#----------------------------------------------------
# G: Termination haldler
def handle_termination(signum, stack_frame_current):
    global running_status
    print(f"Received signal: {signum}")
    running_status = False

signal.signal(signal.SIGTERM, handle_termination) 
signal.signal(signal.SIGINT, handle_termination) 

#----------------------------------------------------
# G: Child Process loop

while running_status and restart_count < max_restart_tries:

    print("Starting child_proc")
    child_proc = subprocess.Popen(command,stdout=stdout_log,stderr=stderr_log,stdin=subprocess.DEVNULL,start_new_session=True)
    exit_code  = child_proc.wait()

    if not running_status:
        break # because running status stoped by handler
    if exit_code == 0: 
        print("Child Exited Naturally. Not Restarting.")
        break # because running status stoped by handler

    print(f"Child exits with exit code {exit_code}")
    restart_count +=1

    if restart_count < max_restart_tries:
        print("Restarting in 3 sec...")
        time.sleep(3)
    else:
        print("Maximum Restarting Attempt Reached.")

#----------------------------------------------------
# G: Child Process terminator which will trigger after loop end & CLEANUP => 

if child_proc and child_proc.poll() is None:
    print(f"Forwarding to child PID: {child_proc.pid}")

    os.killpg(child_proc.pid, signal.SIGTERM) 
    try: 
        child_proc.wait(timeout=5)
    except subprocess.TimeoutExpired :
        print("Child Process ignored SIGTERM. Forcekilling with SIGKILL")
        os.killpg(child_proc.pid, signal.SIGKILL) 
        # Sigkill will directly kill no anyting return TO THE TARGATED PROCESS BUT still SIGCHLD is sent to the parent.
        child_proc.wait()

#----------------------------------------------------

stdout_log.close()
stderr_log.close()

sys.exit(0)
