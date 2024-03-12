# 基础数据结构

* 栈(抽象)
    Push, Pop
* 队列(抽象)
    Dequeue, Enqueue
* 字典(抽象)
    Search, Insert, Delete
    * 字典的实现
        1. 排序数组
            Search $O(log(n))$, Insert $O(n)$, Delete $O(n)$
        2. 无序数组
            Search $O(n)$, Insert $O(1)$, Delete $O(1)$
        3. 排序单链表
            Search $O(n)$, Insert $O(n)$, Delete $O(n)$
        4. 无序单链表
            Search $O(n)$, Insert $O(1)$, Delete $O(n)$
        5. 排序双链表
            Search $O(n)$, Insert $O(n)$, Delete $O(1)$
        6. 无序双链表
            Search $O(n)$, Insert $O(1)$, Delete $O(1)$
* 二叉搜索树(具体)
    Search $O(h)$
    Find-min $O(h)$
    Traversal $O(n)$
    Insert $O(h)$
    Delete $O(h)$
* 平衡二叉树(具体)
    $h = O(log(n))$
* 优先队列(抽象)
    Insert
    Find-min
    Delete-min
    * 优先队列的实现
        1. 无序数组
            Insert $O(1)$
            Find-min $O(1)$
            Delete-min $O(n)$
        2. 排序数组
            Insert $O(n)$
            Find-min $O(1)$
            Delete-min $O(1)$
        3. 平衡二叉树
            Insert $O(log(n))$
            Find-min $O(1)$
            Delete-min $O(log(n))$
* 哈希表(具体)
    插入删除和检索平均复杂度都是 $O(1)$