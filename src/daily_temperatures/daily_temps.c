/******************************************************************************

                            Online C Compiler.
                Code, Compile, Run and Debug C program online.
Write your code in this editor and press "Run" button to compile and execute it.

*******************************************************************************/

#include <iostream>
#include <vector>
#include <optional>

struct Run {
    int &first() {
        return *start;
    }
    
    int &last() {
        return start[len - 1];
    }
    
    /// inclusive pointer to first element.
    const int *start() {
        return start; 
    }
    
    /// inclusive pointer to last element.
    const int *end() {
        return &last();
    }
    
    void extend() {
        len++;
    }
    
    size_t len() {
        return len;
    }
    
    std::optional<int> pop_front() {
        if (len > 0) {
            auto val = first();
            start++;
            len--;
            return val;
        }
        
        return {};
    }
    
    std::optional<int> pop_back() {
        if (len > 0) {
            auto val = last();
            len--;
            return val;
        }
        
        return {};
    }
    
    const int *start{};
    size_t len{};
};


std::vector<int> out dailyTemperatures(const std::vector<int> &in) {

    // dynamic array where we'll be sequentially storing runs.
    auto runs = std::vector<std::optional<Run>>{};
    runs.reserve(in.size());
    
    // arbitrarily tack a run to the first element
    auto current = (Run) { .start = in.data(), .len = 1 };     
    for (size_t i = 1; i < len; i++) {
        // check if the inbound element would prolong the run
        // this is only possible if the inbound number is
        // smaller than the last seen
        if (in[i] <= current.last()) {
            run.extend();
            
        // otherwise, the run is finalized and we save it off.
        // we then create a new run at the current location and repeat.
        } else {
            runs.push_back(current)
            current = (Run) { .start = &in[i], len = 1 };
        }
    }
    
    // now we unwind our run table.
    // because each run is encompassing essentially an in-order
    // and compressed version of the original sequence,
    // we can basically work through the run table and emplace entries
    // directly
    // the last element of a run is 1 day away, and we increment from thereon.
    std::vector<int> out{};
    out.reserve(in.size());
    for (auto j = runs.size(); j >= 0; j--) {
        std::optional<int> val{};
        
        /// start from right because 0 
        /// is the base case and it's on the right
        /// so there are 2 offsets of interest
        // first one is where the run starts
        // second is where in the run we are
        auto &run =  runs[-1];
        auto run_abs_offs = &run.start() - in.data();
        auto out_abs_offs = run_abs_offs;
        auto run_internal_offs = &run.last() - &run.start();
        auto out_internal_offs = run_internal_offs;

        for (auto i = run.end(); i >= run.start(); i--) {
            auto offset_p = &out[out_internal_offs + out_abs_offs + i];
            
            // off the end -> put a zero in offset_p
            if (offset_p >= out.data() + out.size()) {
                *offset_p = 0;
            } else {
                // we're within bounds
                // so you compute the day counter as
                // an increasing sequence from the end
                // to the start of the run.
                *offset_p = run.len() - i;
            }
        }

    }
}

int main()
{
    return 0;
}