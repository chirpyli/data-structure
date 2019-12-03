#include"bstree.h"
// #include"dswtree.h"
// #include"avltree.h"


int main() {
	int data[6] = {8, 3, 10, 1, 4, 6};
	// int data[5] = {13, 10, 15, 5, 1};
	BST<int> *t = new BST<int>(data, sizeof(data)/sizeof(int));
	// t->inorder();
	// t->morrisInorder();
	// std::cout << endl;
	// t->preorder();
	// std::cout << endl;
	t->morrisPreorder();
	// DswBST<int> *bt = new DswBST<int>(data, 6);
	// bt->dswBalance();
	// bt->breadthFirst();

	// AVLTree<int>* at = new AVLTree<int>(data, sizeof(data)/sizeof(int));
	// AVLTree<int>* at = new AVLTree<int>();
	// at->insert(10);
	// at->insert(20);
	// at->insert(30);
	// at->insert(40);
	// at->insert(50);
	// at->insert(25);
	// at->insert(1);
	// at->preOrder(at->getRoot());
	// at->remove(20);
	// std::cout << std::endl;
	// at->preOrder(at->getRoot());


	return 0;
}