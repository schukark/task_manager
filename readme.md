# Pet project in rust: CLI Task Manager

### Now it only implements some basic functionality:
1. Add/remove task
2. Mark task completed
3. List all tasks
4. Saves the tasks on disk using serde to have some persistence
5. Update task (title/description/date)
6. Utilized `clap` to make the interface more user-friendly
7. Documentation added (comments throughout the code)

### TODO:
1. Some additional features like sorting based on different criteria
2. Add priority to tasks
3. Enhance the interface, because it looks ugly right now

### Plans after finishing
As it is a pet-project to get the hang of rust, the next logical step would be to extend the project for it to work in web using something like `actix`