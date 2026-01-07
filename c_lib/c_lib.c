#include "c_lib.h"

static struct Data *STORED = 0;

void c_take(struct Data *d) {
    STORED = d;
}

void c_increment(void) {
    if (STORED) {
        STORED->value += 1;
    }
}

struct Data *c_return(void) {
    struct Data *tmp = STORED;
    STORED = 0;
    return tmp;
}
