#include<cassert>
#include<iostream>
#include<stack>
using namespace std;

template<class T>
struct Node {
    T data;
    Node* next;
};

template<class T>
void print_list(Node<T>* head) {
    cout << "List: ";
    while (head != nullptr) {
        cout << head->data << " ";
        head = head->next;
    }
    cout << endl;
}

template<class T>
Node<T>* construct_list(int n) {
	Node<T>* head = new Node<T>;
	head->data = 0;
	head->next = nullptr;
	Node<T>* tmp = head;
	for (int i = 1; i < n; i++) {
		Node<T>* node = new Node<T>;
		node->data = i;
		node->next = nullptr;
		tmp->next = node;
		tmp = node;
	}

	return head;
}


// 如何实现单链表反转？
template<class T>
Node<T>* converse_list(Node<T>* head) {
    std::stack<Node<T>*> m_stack;
    while (head != nullptr) {
        m_stack.push(head);
        head = head->next;
    }

    Node<T>* node = m_stack.top();
    Node<T>* rhead = node;
    m_stack.pop();
    while (!m_stack.empty()) {
        node->next = m_stack.top();
        m_stack.pop();
        node = node->next;
    }
    node->next = nullptr;

    return rhead;
}



int main() {
    Node<int>* head = construct_list<int>(10);
    print_list<int>(head);
    Node<int>* rehead = converse_list<int>(head);
    print_list<int>(rehead);

    return 0;
}