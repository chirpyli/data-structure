#include"bstree.h"
#include"dswtree.h"



int main() {
	int data[6] = {8, 3, 10, 1, 4, 6};
	BST<int> *t = new BST<int>(data, 6, true);
	// t->inorder();
	t->morrisInorder();
	std::cout << endl;
	t->breadthFirst();
	std::cout << endl;

	DswBST<int> *bt = new DswBST<int>(data, 6);
	bt->dswBalance();
	bt->breadthFirst();


	return 0;
}