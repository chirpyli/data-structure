在图的应用中，最常用的就是求解各种条件下的最短路径问题，这里Dijkstra算法是求解有权（权值为非负值）图的单源最短路径算法，即只能求出某点到其它点最短距离，并不能得出任意两点之间的最短距离。

经典实现伪代码如下：
```
 1  function Dijkstra(Graph, source):
 2
 3      create vertex set Q
 4
 5      for each vertex v in Graph:             
 6          dist[v] ← INFINITY                  
 7          prev[v] ← UNDEFINED                 
 8          add v to Q                      
10      dist[source] ← 0                        
11      
12      while Q is not empty:
13          u ← vertex in Q with min dist[u]    
14                                              
15          remove u from Q 
16          
17          for each neighbor v of u:           // only v that are still in Q
18              alt ← dist[u] + length(u, v)
19              if alt < dist[v]:               
20                  dist[v] ← alt 
21                  prev[v] ← u 
22
23      return dist[], prev[]
```
可以看到Dijkstra算法核心思想是每次都从距离值最短的顶点开始探索最短路径，从当前顶点探索更新完自己的邻接节点后，如果发现有其他顶点的距离值更小，则跳至那个顶点，继续探索，直到所有顶点都被处理过。其算法过程可参考下图：
> 如果有负权边，每经过一次负权边，路径长度都会减小，该算法就会陷入死循环。

![image](.images/Dijkstra_Animation.gif)

上面会计算出源结点到其他结点的最短距离，如果是要获得源节点到目的节点之间的最短路径，伪代码如下：
```
1  S ← empty sequence
2  u ← target
3  if prev[u] is defined or u = source:          // Do something only if the vertex is reachable
4      while u is defined:                       // Construct the shortest path with a stack S
5          insert u at the beginning of S        // Push the vertex onto the stack
6          u ← prev[u]                           // Traverse from target to source
```
因为上面每次都会在发现更短的路径后更新前驱节点（上一跳节点），所以，获取最短路径只要从目的节点依次找其前驱节点，就等于逆序输出了最短路径。

Dijkstra算法（包括经典实现和优先级队列实现）完整的C++代码实现参见[dijkstra.cpp](../codes/graph/dijkstra.cpp)

>最后需要注意Dijkstra的适用性，它不适用于包含负权边的图。因为Dijkstra算法基于这样的假设：对于处理过的节点，没有前往该节点的更短路径。这种假设仅在没有负权边时才成立， 如果有负权边，每经过一次负权边，路径长度都会减小，该算法下会陷入死循环。因此，不能将Dijkstra's algorithm用于包含负权边的图。


>参考文档：[Dijkstra's algorithm](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm)