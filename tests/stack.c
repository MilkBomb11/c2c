#include<stdlib.h>

struct Stack {
	struct Node *top;
	int size;
};

struct Node {
	struct Node *next;
	int val;
};

void stack_init(struct Stack *stack) {
	stack->top = NULL;
	stack->size = 0;
}

void stack_push(struct Stack *stack, int val) {
	struct Node *node = (struct Node *)malloc(sizeof(struct Node ));
	node->val = val;
	node->next = stack->top;
	stack->top = node;
}
