

#[test]
fn it_should_be_refactor_by_using_a_closure_instead_of_function_pointer() {
    //TODO: change guard in a closure
    let guard = |a| a >= 10;

    let res = guard(22);

    assert!(res);
}

#[test]
fn it_should_can_be_use_in_a_map() {
    fn add_one(a: u8) -> u8 {
        a + 1
    }

    let vals = vec![1, 2];

    let new_vals: Vec<_> = vals.into_iter()
                           //TODO: use a map which use the add_one in a closure
                               .map(|v| add_one(v))
                               .collect();

    assert_eq!(vec![2, 3], new_vals);
}

#[test]
fn it_should_pass_the_correct_closure() {
    struct Task(u8);

    #[derive(Debug, PartialEq)]
    struct TaskFinish(u8);

    fn processing_task<F>(process_task: F, task: Task) -> TaskFinish
        where F: Fn(Task) -> TaskFinish,
    {
        process_task(task)
    }

    let task = Task(1);

    //TODO: call processing_task by passing it your closure.
    let task_finish = processing_task(
        |t| match t {
            Task(id) => TaskFinish(id + 1),
        },
        task,
    );

    assert_eq!(task_finish, TaskFinish(2));
}
