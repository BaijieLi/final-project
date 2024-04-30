DS 210 Final Project Report

For my project, I use the dataset by the following link 
“https://www.kaggle.com/datasets/noahgift/social-power-nba” in kaggle.com. I combined two csv together and then collected the necessary data such as “player name”, “team name”, “Assists number”, “Salary in millions”, and Twitter favorite count”.  We are going to use them to find the average distance, closeness centrality, and players with similar pairs.
Average distance
Average Euclidean Distance: According to the command output, the average distance between players is based on their salary in millions and their Twitter favorite count, I get the average distance is 5.00. 
The smallest distance observed between any two players shows the closest pair regarding salary and Twitter favorites. The players in this pair have the most similar salary and Twitter favorite count. The largest distance observed shows the pair of players that are the most different from each other in terms of salary and Twitter favorites.
The shortest distance recorded is 0, which we observed multiple times, one of the distances recorded is 0 is between Russell Westbrook and Kevin Durant.
The longest distance found is 21, between Georgios Papagiannis and Vince Carter. This shows a dissimilarity or difference between these two players concerning the attributes measured.
As a result of this output, we could analyze that if Russell Westbrook and Kevin Durant are not on the same team, and both team manager wants to trade them, their trade value is the same, there is no need to add more first-round picks or pay more salary to make the exchange successful.
Closeness Centrality
"There are 30 connected components" means that the graph consists of 30 different groups of nodes (players), where each group is connected internally, but there are no connections between different groups. This indicates that players are grouped by team because there are 30 teams in the NBA.
"Top 20 Closeness Centrality" means that these are the top 20 players with the highest closeness centrality values. For example, "Node James Harden: Closeness Centrality = 59.2500" means that James Harden is the core player in his component, which means that he can quickly connect to other players in his network component. In other words, James Harden may be the leader or captain of this team. The tactics the coach arranges for other players on the team are all based on how James Harden successfully scores. James Harden is also the key player and cornerstone of this team. 
Players with similar pair
I use petgraph to construct the graph, Edge is the "similar assists", node is the "player", and the difference between two players in Top 20 Closeness Centrality with assists number below 5 or less, I mark them as similar assists. and combine them as a pair. Their relationships are undirected, meaning the relationship doesn't have a direction or order. The assists similarity is the same regardless of which player is considered first.






