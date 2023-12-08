# How the tools works
For this softwaire fair we created two tools which will simplify your robot work...
## Shortest path GPS:
This tool garantee you can to find the shortest path to a specific block known. 
In fact, by using the ```robot_map``` function (the map that the robot has already seen),
we create the shortest path to the position selected.

***In the next test we will consider that the robot has already explored all the world, so
the ```robot_map``` function can see every block.***
### Function:
```fn gps (robot: &impl Runnable, destination: (usize,usize), world: &World,) -> Option<(Vec<Direction>, usize)> ```
## Here there are a few examples: (We got also the code 😉)

### Example number 1:
In the file ```show_example1.rs``` we implemented this world:

<p align="center">
    <img src="images/show_example1_begin.png">
</p>

If we want to go to the destination at the coordinates```(3,2)``` we can have multiple
path to got there. \
Our algorithm will manage to find the **best path** to arrive at the specified position, trying to **lose
as little energy as possible**. \

The best path for reaching the block is the one in the picture below:

<p align="center">
    <img src="images/show_example1_path.png">
</p>

Since the previous example was easier, we also got something more challeging for our robot.
### Example number 2:

In the file ```show_example_teleport.rs``` we implemented this world: \
Our algorithm also implement the usage of the teleport tiletype, whether they are enable.
<p align="center">
    <img src="images/show_example_teleport_begin.png">
</p>

In this example we can have 2 general path to reach 

<p align="center">
    <img src="images/show_example_teleport_path.png">
</p>
