#include<iostream>

namespace mytree {

using namespace std;

enum ecolor {
    red,
    black,
};

template<class T>
struct Node {
    Node(T v):data(v),color(ecolor::red),left(nullptr),right(nullptr),parent(nullptr){}
    
    Node<T>* grandparent() {
        if (nullptr == parent)
            return nullptr;
        return parent->parent;
    }

    Node<T>* uncle() {
        if (nullptr == grandparent())
            return nullptr;
        if (grandparent()->left == parent) 
            return grandparent()->right;
        else 
            return grandparent()->left;
    }
    
    T data;
    ecolor color;
    Node<T>* left;
    Node<T>* right;
    Node<T>* parent;


};

template<class T>
class RBTree {
public:
    RBTree():root(nullptr),nil(nullptr){}

    void insert(T e);

private:
    Node<T>* root;
    Node<T>* nil;
};




}