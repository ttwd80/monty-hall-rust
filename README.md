# monty-hall-rust
Monty Hall in Rust

I watched this https://m.youtube.com/watch?v=cXqDIFUB7YU

After watching it and listening to the explanation, I still did not fully understand it.

It took a while but now I understand it.

Step 1: Three doors. Two are bad. One is good.

Step 2: Pick a door. Chances of picking a good one is 33.33%. Chances of getting a bad door is 66.67%.

Step 3: One of the bad doors is taken out. Leaving one good door and one bad door.

Step 4: Here is the tricky part. There are 2 doors now. Either you stick with your chosen door in Step 2 or change to a different door. Since there are 2 doors, one is good and one is bad. If you picked a door and you change it, if the initial choice was a good door, you are going to get a bad door now. And if the initial choice was a bad door, you are going to get a good door now. Looking back at step 2, you can see that chances of getting a bad door in step 2 is 66.67% (2 out of 3 you are going to pick a bad door). Changing your answer here, good becomes bad and bad becomes good. The original 66.67% of picking a bad door now becomes 66.67% of picking a good door.

