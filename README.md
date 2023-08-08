# ReMMIX
Interpreter for MMIX in Rust

# Resources
[Art of Computer Programming, Volume 1, Fascicle 1, The: MMIX -- A RISC Computer for the New Millennium](https://learning.oreilly.com/library/view/art-of-computer/9780321637369/)

[MMIXware](https://link.springer.com/book/10.1007/3-540-46611-8)

# Notes
MIXX is always big-endian.

M[k] to address kth byte. This generalizes to M_(2^t)[k]. M[k] = M_(2^0)[k].

M_(2^t)[k] addresses the 2^t consecutive bytes starting at (k ^ (2^64 - 2^t)).
The above zeroes out the least significant t bits of k, and the least 64 bit of the address is retained.

(k v (2^t - 1)) means the least significant t bits are set to 1.

All access to 2^t-byte quantities are aligned -- first byte in the addressed region is a multiple of 2^t.

M_(2^t)[k] is the region between M[k ^ (2^64 - 2^t)] and M[k v (2^t - 1)], inclusive.

If l = 2^t, then the above becomes:
M_(l)[k] is the region between M[k ^ (2^64 - l)] and M[k v (l - 1)], inclusive.

s(M_(l)[k]) = interpret the l (2^t) byte region as a signed l byte number.

s(M_(l)[k]) = (M[k ^ (2^64 - l)]M[k ^ (2^64 - l) + 1]...M[k v (l - 1)]) - 2^(8l)*(M[k ^ (2^64 - l)] >= 128)