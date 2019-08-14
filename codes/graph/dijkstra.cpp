// Dijkstra最短路径算法

#include <cassert>
#include<vector>
#include<list>
#include<iostream>
#include<utility> 
#include<climits>
#include<algorithm>
using namespace std;


// 无向有权（非负值）图
class Graph {
public:
    Graph(int numVertices) {
        this->numVertices = numVertices;
        matrix = new int*[numVertices];
        for (int i = 0; i < numVertices; ++i) {
            matrix[i] = new int[numVertices];
            for (int j = 0; j < numVertices; ++j) {
                matrix[i][j] = 0;
            }
        }
    }

    void addEdge(int v1, int v2, int value = 1) {
        matrix[v1][v2] = value;
        matrix[v2][v1] = value;
    }

    void removeEdge(int v1, int v2) {
        matrix[v1][v2] = 0;
        matrix[v2][v1] = 0;
    }

    list<int> dijkstraShortestPath(int src, int dst) {
        assert(src != dst);
        vector<int> prev = dijkstra(src);
        list<int> path;
        int u = dst;
        if (prev[u] != -1) {
            while (prev[u] != -1) {
                path.push_front(u);
                u = prev[u];    
            }
        }

        return path;
    }

    vector<int> dijkstra(int source) {
        vector<int> prev;
        vector<int> dist;
        list<int> vlist;

        for (int i = 0; i < this->numVertices; ++i) {
            dist.push_back(INT_MAX);
            prev.push_back(-1);
            vlist.push_back(i);
        }
        dist[source] = 0;
 
        while (!vlist.empty()) {
            int u =  minVertexDistance(&vlist, &dist);
            vlist.remove(u);
            vector<int> neighbors = neighbor_vertices(u);
            for (auto it = neighbors.begin(); it != neighbors.end(); ++it) {
                int alt = dist[u] + distance(u, *it);
                if (alt < dist[*it]) {
                    dist[*it] = alt;
                    prev[*it] = u;
                }
            }
        }

        // 打印距离值
        for (int i = 0; i < numVertices; ++i) {
            cout << i << " : " << dist[i] << endl;
        }

         return prev;
    }

    vector<int> neighbor_vertices(int vertex) {
        vector<int> neighbor;
        for (int i = 0; i < numVertices; ++i) {
            if (matrix[vertex][i] > 0) {
                neighbor.push_back(i);
            }
        }

        return neighbor;
    }

    int distance(int v1, int v2) {
        return matrix[v1][v2];
    }

    void print() {
        for (int i = 0; i < numVertices; ++i) {
            cout << i << " : ";
            for (int j = 0; j < numVertices; ++j) {
                cout << matrix[i][j] << " ";
            }
            cout << endl;
        } 
    }

    ~Graph() {
        for (int i = 0; i < numVertices; ++i) {
            delete[] matrix[i];
        }

        delete[] matrix;
    }

private: 
    int **matrix;
    int numVertices;

    bool isEdge(int v1, int v2) {
        return matrix[v1][v2] >= 0 ? true : false ;
    }

    int minVertexDistance(list<int> *vlist, vector<int> *dist) {
        assert(!vlist->empty() && !dist->empty());
        auto it = vlist->begin();
        int v = *it;
        int min = (*dist)[v];
        for ( ;it!= vlist->end(); ++it) {
            if ((*dist)[*it] < min) {
                v = *it;
            }
        }

        return v;
    }
};



/*
邻接矩阵
0 : 0 3 2 0 0 
1 : 3 0 0 1 0 
2 : 2 0 0 5 8 
3 : 0 1 5 0 4 
4 : 0 0 8 4 0 

距离
0 : 0
1 : 3
2 : 2
3 : 4
4 : 8

最短路径：path: 1 3 4
 */
void test() {
    Graph g(5);
    g.addEdge(0, 1, 3); 
    g.addEdge(0, 2, 2); 
    g.addEdge(1, 3, 1); 
    g.addEdge(2, 3, 5); 
    g.addEdge(2, 4, 8); 
    g.addEdge(3, 4, 4); 
    g.print();

    vector<int> prev = g.dijkstra(0);
    list<int> path = g.dijkstraShortestPath(0, 4);
    cout << "path: ";
    for (auto it = path.begin(); it != path.end(); ++it) {
        cout << *it << " ";
    }
}

int main() {
    test();

    return 0;
}