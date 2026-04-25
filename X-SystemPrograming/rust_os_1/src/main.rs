use nix::unistd::{fork, ForkResult, getpid, getppid, execvp};
use nix::sys::wait::wait;
use std::ffi::CString;

fn main() {
    get_parent_child_pids();
    fork_demostration_funciton();
    fork_execvp_wait_function();
}


fn get_parent_child_pids(){

    match unsafe{fork()}{
        Ok(ForkResult::Child) => {
            println!("Child PID: {}",getpid());
        }
        Ok(ForkResult::Parent { child }) => {
            println!("Parent PID: {}", getpid());
            println!("Child PID: {}", child);
        }
        Err(_) => {println!("Fork Failed")}
    }
    println!("=======================================================")

// Y: Practice: fork exec wait pipe dup epoll |
// Trace them using: strace ls
// strace ./target/debug/<ProgrameName>
}

fn fork_demostration_funciton(){
    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            println!("👶 Child Process");
            println!("Child PID: {}", getpid());
            println!("Parent PID: {}", getppid());
        }

        Ok(ForkResult::Parent { child }) => {
            println!("👨 Parent Process");
            println!("Parent PID: {}", getpid());
            println!("Child PID: {}", child);

            wait().expect("Failed to wait");
            println!("Parent: Child finished execution");
        }

        Err(_) => println!("Fork failed"),
    }

    println!("=======================================================X")
}

fn fork_execvp_wait_function() {
    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            println!("Child: About to execute ls");

            let cmd = CString::new("ls").unwrap();
            let args = [cmd.clone()];

            execvp(&cmd, &args).expect("Exec failed");
        }

        Ok(ForkResult::Parent { child }) => {
            println!("Parent: Waiting for child {}", child);
            wait().expect("Wait failed");
            println!("Parent: Child completed");
        }

        Err(_) => println!("Fork failed"),
    }

    println!("=======================================================Y")
}
