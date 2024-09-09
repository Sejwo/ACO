Hello traveller, 

So essentially this is a very basic application of ACO algorithm, still could use some polishing to be a real crate, right now I still need to work on some minute changes on it, and other improvements.
I'm fairly new to Rust so some things obviously could be optimised by leveraging the languages possibilites.
Also the code gets spaghetti-ish at times which is what I'd mainly optimise and comment properly because if not for a really well written project in Python I found
(I think citation got lost in the commit history, will need to address that) I would have been done for around the logic and implementation.
I kind of want it to be a "real crate" so that it can work outside of strict boundries I've set to get it to work in the first place.
To get it to a perfect Rustic shape, it will take time and a long evening(s) with the handbook.

Treat the below as a loose todo list:
-I will definately come back to this but as is it's able to solve most easy problems,with the more complex ones I will have to do some tuning to the decision/pheromone dispersion algorithm:
  - I'm well aware the current power of 1.1 solution is faulty at best
  - I was thinking maybe borrowing some easings functions from the JS community could perhaps make the changes more plastic (i.e. slower progression with short stagnation, bigger progression with with longer stagnation)
  - Re-evalutating the 2D array approach, it just seems needlesly repetitive to apply the pheromones to both directions on the matrix (i.e. after an ant passes from city 1 to city 5 i need to apply pheromone to both [1][5] and [5][1] 

-If I had some cash I'd probably try using google street maps API to better map distances between places
(namely. rather than using geographic distance in straight line i could use how long is the distance for commuting between cities)
but at the same time I would have to consider not necessarily distance but time and that's yet another can of worms I don't want to deal with right now.





