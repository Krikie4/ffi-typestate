#pragma once

struct Data {
    int value;
};

void c_take(struct Data *d);
void c_increment(void);
struct Data *c_return(void);
