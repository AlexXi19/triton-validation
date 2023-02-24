mod k8s;

fn main() {
    k8s::scheduler::schedule();
}
