
// #要求：请不要使用系统库、第三方库提供的便捷功能，包括排序。如有需要，请自行实现。

// #归并排序。给定两个有序数组（整数、递增），请将它们归并成一个有序数组。
// #例：
// #输入：[1,3,5,7], [2,4,6,8]
// #输出：[1,2,3,5,6,7,8]

#include<cassert>
#include<iostream>
using namespace std;

void merge_sort(int* a, int *b, const int alen, const int blen, int c[]) {
    assert(a != nullptr || b != nullptr);
    int pos_a = 0;
    int pos_b = 0;
    int pos = 0;
    while(pos_a < alen && pos_b < blen) {
        if (a[pos_a] < b[pos_b]) {
            c[pos++] = a[pos_a++];
        } else {
            c[pos++] = b[pos_b++];
        }
    }

    if (pos_a == alen) {
        while(pos_b < blen) {
            c[pos++] = b[pos_b++];
        }
    } else if (pos_b == blen) {
        while(pos_a < alen) {
            c[pos++] = a[pos_a++];
        }
    }
}

int main() {
    int a[7] = {1,3,5,7,9,11,21};
    int b[4] = {2,4,6,8};
    int c[11];
    merge_sort(a, b, 7, 4, c);
    for(auto i : c) {
        cout << i << endl;
    }

    return 0;
}