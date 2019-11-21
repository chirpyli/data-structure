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

//跳跃列表节点类
template<class K, class V>
class SkipListNode {
public:
    SkipListNode() {}
    SkipListNode(K key, V value, int level);
    K key;
    V value;
    typedef SkipListNode<K, V>* NodePtr;
    vector<NodePtr> forward;
};

template<class K, class V>
SkipListNode<K, V>::SkipListNode(K key, V value, int level):key(key),value(value) {
    for (int i = 0; i < level; ++i) {
        this->forward.push_back(nullptr);
    }
}

// 跳跃列表类
template<class K, class V>
class SkipList {
public:
    SkipList();
    ~SkipList();
    V* search(const K& key);
    void insert(const K& key,const V& value);
    void remove(const K& key);
    void print();
private:
    int randomLevel() const;
    typedef SkipListNode<K, V>* NodePtr;
    vector<NodePtr> header;   // header
    int level;                // 当前层数
    int n;                    // 节点数量
};

template<class K, class V>
void SkipList<K, V>::print() {
    std::cout << "level=" << this->level << " count=" << this->n << endl;
    if (this->level == 0)
        return;
    for (int i = this->level -1; i >=0 ; --i) {
        cout << "level" << i + 1 << ":";
        NodePtr pNode = header[i];
        while (pNode != nullptr) {
            cout << pNode->value << " ";
            pNode = pNode->forward[i]; 
        }
        cout << endl;
    }
}

template<class K, class V>
SkipList<K, V>::SkipList() {
    // this->header.push_back(nullptr);
    this->level = 0;
    this->n = 0;
}

template<class K, class V>
SkipList<K, V>::~SkipList() {
}

template<class K, class V>
int SkipList<K, V>::randomLevel() const {
    int lvl = 1;
    int maxLevel = (int)log(this->n) + 1;
    while (rand() % 100 < P * 100 && lvl < min(maxLevel, MAX_LEVEL)) {
        ++lvl;
    }

    return lvl;
}

// template<class K, class V>
// V* SkipList<K, V>::search(const K& key) {
//     NodePtr x = nullptr;
//     for (int i = this->level - 1; i >=0; --i) { 
//         x = this->header[i];
//         if (nullptr == x)
//             continue;
//         if (key == x->key) {
//             return &x->value;
//         } else if (key > x->key) {
//             break;
//         } 
//     }
//     if (nullptr == x)
//         return nullptr;
//     for (int i = this->level - 1; i >= 0; --i) {
//         while (x->forward[i] != nullptr && x->forward[i]->key < key) {
//             x = x->forward[i];
//         }       
//     }
//     x = x->forward[0];      //论文中实现是查到最底层再返回，自己实现也可以不用找到最底层就返回，这里采用论文中的算法
//     if (x->key == key) {
//         return &x->value;
//     } else {
//         return nullptr;
//     }
// }

template<class K, class V>
void SkipList<K, V>::insert(const K& key,const V& value) {
    // 随机生成该节点层数，调整header
    const int lvl = this->randomLevel();
    cout << "random level : " << lvl << endl;
    if (lvl > this->level) {
        for (int i = this->level; i < lvl; ++i) {
            this->header.push_back(nullptr);
        }
        this->level = lvl;
    }

    // 找到待调整的项
    NodePtr update[this->level];
    int slvl = this->level - 1;
    NodePtr x = this->header.at(slvl);

    for (; slvl >= 0; ) {
        if (x != nullptr) {
            if (key == x->key) {
                x->value = value;
                return;
            } else if (key < x->key) {
                update[slvl] = nullptr;
                if (0 == slvl)
                    break;
                x = this->header.at(--slvl);
            } else {
                while (x->forward[slvl] != nullptr && x->forward[slvl]->key < key) {
                    x = x->forward[slvl];
                }
                update[slvl--] = x;
            }
        } else {
            update[slvl] = nullptr;
            if (slvl == 0)
                break;
            x = this->header.at(--slvl);
        }
    }

    // 进行节点加入操作（与链表插入类似）
    x = new SkipListNode<K, V>(key, value, lvl);
    for (int i = 0; i < lvl; ++i) {
        if (nullptr == update[i]) {
            if (this->header[i] != nullptr) {
                x->forward[i] = this->header[i]->forward[i];
            } else {
                x->forward[i] = nullptr;
            }
            this->header[i] = x;
        } else {
            x->forward[i] = update[i]->forward[i];
            update[i]->forward[i] = x;
        }
    }
    this->n++;
}


int main() {
    srand(time(nullptr));
    SkipList<int, int>* pSkiplist = new SkipList<int, int>();
    pSkiplist->insert(10, 10);
    pSkiplist->insert(15, 15);
    pSkiplist->insert(12, 12);
    pSkiplist->insert(18, 18);
    pSkiplist->insert(1, 1);

    pSkiplist->print();

    delete pSkiplist;

}
