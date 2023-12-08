# How the tools works
For this softwaire fair we created two tools which will simplify your robot work.
## Shortest path GPS:
This tool garantee you can to find the shortest path to a specific block known. 
In fact, by using the ```robot_map``` function (the map that the robot has already seen),
we create the shortest path to the position selected.\
### Function:
```fn gps (robot: &impl Runnable, destination: (usize,usize), world: &World,) -> Option<(Vec<Direction>, usize)> ```
### Here there are a few examples: (We got also the code 😉)
In the file ```show_example1.rs``` we implemented this world with the relatives costs:

![Immagine](images/Bulk_friend.png)
