#include<iostream>
#include<cstdlib>
#include<ctime>
#include<random>
#include<cmath>
#include<vector>
#include<algorithm>

using namespace std;

const int MAX_LEVEL = 10;   // 用户定义的最大层数
const float P = 0.5;        // 用户设置的概率P

template<class K, class V>
class SkipListNode {
public:
    SkipListNode() {}
    SkipListNode(K key, V value, int level);
    K key;
    V value;
    SkipListNode* forward[];
};

template<class K, class V>
SkipListNode<K, V>::SkipListNode(K key, V value, int level):key(key),value(value) {
    this->forward = new SkipListNode*[level];
    for (int i = 0; i < level; ++i) {
        this->forward[i] = NULL;
    }
}

template<class K, class V>
class SkipList {
public:
    SkipList();
    ~SkipList();
    V* search(const K& key);
    void insert(const K& key,const V& value);
    void remove(const K& key);
private:
    int randomLevel() const;
    typedef SkipListNode<K, V>* NodePtr;
    vector<NodePtr> header;   // header
    int level;                // 当前层数
    int n;                    // 节点数量
};

template<class K, class V>
SkipList<K, V>::SkipList() {
    this->header.push_back(NULL);
    this->level = 1;
    this->n = 0;
}

template<class K, class V>
SkipList<K, V>::~SkipList() {
}

template<class K, class V>
int SkipList<K, V>::randomLevel() const{
    int lvl = 1;
    int maxLevel = (int)log(this->n) + 1;
    while (rand() % 100 < P * 100 && lvl < min(maxLevel, MAX_LEVEL)) {
        ++lvl;
    }

    return lvl;
}

template<class K, class V>
V* SkipList<K, V>::search(const K& key) {
    NodePtr x = NULL;
    for (int i = this->level - 1; i >=0; --i) {
        x = this->header[i];
        if (NULL == x)
            continue;
        if (key == x->key) {
            return &x->value;
        } else if (key > x->key) {
            break;
        } 
    }
    if (NULL == x)
        return NULL;
    for (int i = this->level - 1; i >= 0; --i) {
        while (x->forward[i] != NULL && x->forward[i]->key < key) {
            x = x->forward[i];
        }       
    }
    x = x->forward[0];      //论文中实现是查到最底层再返回，自己实现也可以不用找到最底层就返回，这里采用论文中的算法
    if (x->key == key) {
        return &x->value;
    } else {
        return NULL;
    }
}

template<class K, class V>
void SkipList<K, V>::insert(const K& key,const V& value) {
    // 随机生成该节点层数，调整header
    const int lvl = this->randomLevel();
    if (lvl > this->level) {
        for (int i = this->level; i < lvl; ++i) {
            this->header.push_back(NULL);
        }
        this->level = lvl;
    }

    // 找到待调整的项
    NodePtr update[this->level];
    NodePtr x = NULL;
    int slvl = this->level - 1;
    for (; slvl >=0; --slvl) {
        x = this->header[slvl];
        if (NULL == x)
            update[slvl] = x;
            continue;
        if (key == x->key) {
            x->value = value;
            return;
        } else if (key < x->key) {
            update[slvl] = x;
        } else {
            break;
        }
    }

    for (; slvl >= 0; --slvl) {
        while (x->forward[slvl] != NULL && x->forward[slvl]->key < key) {
            x = x->forward[slvl];
        }
        update[slvl] = x;       
    }
    x = x->forward[0];      
    if (x->key == key) {
        x->value = value;
    } else {
        
    }

}


int main() {
    srand(time(NULL));
}
