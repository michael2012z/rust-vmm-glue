# rust-vmm-glue

Rust-VMM reference implementation prototype.

## Short Description
The reference implementation bears 2 major targets:
* An example to show how to use Rust-VMM.
* Rust-VMM components integration test.

### Design
#### Principles of the design

- Simple
  - The architecture of the reference implementation should be simple. It should be easy for people to understand how rust-vmm components are connected to build a hypervisor. 
  - A good reference is kvmtool (https://git.kernel.org/pub/scm/linux/kernel/git/will/kvmtool.git), it is "a clean, from-scratch, lightweight KVM host tool". This reference shares the same goal. 
  - To keep simple, the reference program is not designed in centerlization mode: no daemon, no API server.

- General purposed
  - The main consuming projects of rust-vmm (Firecracker and Clould-Hypervisor now, CrosVM may in future) each has its own character and focus. But this reference implementation should be general purposed, it should cover variant use cases. For example, it should support devices commonly used as many as possible.
  - Integrate all functional components (crates).

#### Practical Design
- Prototype
  - I am making a prototype of the design at: https://github.com/michael2012z/rust-vmm-glue
- Binary
  - The reference implementation was built into a single binary file. I assume the binary is named as "glue" (means glue of rust-vmm components) in following description.

- Subcommands
  - The reference implementation works in the way of subcommands. The command format looks like:

  - > ``` glue [FLAGS] [subcommand] [OPTIONS] ```

- Process diagram of RUN subcommand
  - ![](https://raw.githubusercontent.com/michael2012z/rust-vmm-glue/master/docs/images/cmd_run.png "RUN subcommand")

- Usage examples

HELP subcommand
```
$ ./target/debug/glue --help
glue 0.1
Rust-VMM based hypervisor.

USAGE:
    glue [FLAGS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Sets the level of verbosity

SUBCOMMANDS:
    help      Prints this message or the help of the given subcommand(s)
    pause     Pause the virtual machine
    resume    Resume the virtual machine
    run       Start the virtual machine
    stop      Stop the virtual machine
    list      List all virtual machines

```

RUN subcommand
```
$ ./target/debug/glue run --help
glue-run 
Start the virtual machine

USAGE:
    glue run [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --cpus <cpus>        Number of CPUs [default: 1]
    -d, --disk <FILE>        Disk image
    -k, --kernel <FILE>      Kernel to boot
    -m, --mem <mem>          Memory size in MB [default: 512]
    -n, --name <name>        A name for the VM
    -p, --params <params>    Kernel command line arguments
```

PAUSE subcommand
```
$ ./target/debug/glue pause --help
glue-pause 
Pause the virtual machine

USAGE:
    glue pause [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --name <name>    Name of the VM
```

### Integration test

#### Cover as many use cases as possible
- Build - Customize hypervisor (glue) by specifying testing manifest file (it's Cargo.toml by default) with different features/components and build. 
  - >  ``` cargo build --manifest-path ./tests/build/example.toml ``` 

- Basic - Basic functions of VM.
  - Lifecycle
  - Kernel load
  - Kernel command line
  - RootFS load
  - vcpus
  - Memory
  - ...
  
- Devices - Device integration tests.
  - PCI
  - GPU
  - Combinations
  - ...
  
- Features - Features of rust-vmm components and their combinations.
  - To be scoped.
  
- Performance - Performance tests, typically:
  - Boot time
  - Process start time
  - Device IO time
  - ...

#### Test framework candidates
This is to be discussed. Should we use Pytest like Firecracker do or use Rust test?
- Pytest (I prefer this)
  - "Fixture" is a powerful feature of Pytest, which makes it easy to prepare a VM instance with proper capability (like netowrk, network, etc) for test case.
  - Firecraker has a good test codebase writen in Pytest for reference.
  - Python is easy to use and to integrate in CI.
- Rust
  - Using Rust native test is an alternative solution. 
  - Align the test code with function code looks good.
  - We need to implement a test framework from scratch. I didn't know much about popular Rust test framework. If there is, things could be different.
