#include<iostream>
#include<cassert>

using namespace std;

template<class T>
void print_array(const T* a, const int n) {
    cout << "array: ";
    for (int i = 0; i < n; ++i) {
        cout << *(a + i) << " ";
    }
    cout << endl;
}

// 冒泡排序法
template<class T>
void bubble_sort(T* a, const int n) {
    T tmp;
    for (int i = 1; i < n; ++i) {
        for (int j = n - 1; j >= i; --j) {
            if (a[j] < a[j-1]) {
                tmp = a[j-1];
                a[j-1] = a[j];
                a[j] = tmp;
            }
        }
    }
}

// 双向冒泡法
template<class T>
void double_bubble_sort(T* a, const int n) {
    T tmp;
    int left = 1;
    int right = n - 1;
    int t;
    do {
        // 正向部分
        for (int i = right; i >= left; --i) {
            if (a[i] < a[i-1]) {
                tmp = a[i-1];
                a[i-1] = a[i];
                a[i] = tmp;
                t = i;
            }
        }
        left = t + 1;
        // 反向部分
        for (int i = left; i < right + 1; ++i) {
            if (a[i] < a[i-1]) {
                tmp = a[i];
                a[i] = a[i-1];
                a[i-1] = tmp;
                t = i;
            }    
        }
        right = t - 1;
    } while (left <= right);
}

// 交换排序法（每次用当前的元素一一的同其后的元素比较并交换）
template<class T>
void swap_sort(T* a, const int n) {
    T tmp;
    for (int i = 0; i < n - 1; ++i) {
        for (int j = i + 1; j < n; ++j) {
            if (a[i] > a[j]) {
                tmp = a[i];
                a[i] = a[j];
                a[j] = tmp;
            }
        }
    }
}

// 选择排序法
template<class T>
void select_sort(T* a, const int n) {
    T tmp;
    int pos;
    for (int i = 0; i < n - 1; ++i) {
        T min = a[i];
        pos = i;
        for (int j = i + 1; j < n; ++j) {
            if (a[j] < min) {
                min = a[j];
                pos = j;
            }
        }
        tmp = a[i];
        a[i] = min;
        a[pos] = tmp;
    }
}

// 插入排序法
template<class T>














// 快速排序法




int main() {
    int a[] = {1,4,6,3,9,3,2};
    // bubble_sort<int>(a, sizeof(a)/sizeof(int));
    // double_bubble_sort<int>(a, sizeof(a)/sizeof(int));
    // swap_sort<int>(a, sizeof(a)/sizeof(int));
    select_sort<int>(a, sizeof(a)/sizeof(int));
    print_array<int>(a, sizeof(a)/sizeof(int));
}