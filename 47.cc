#include <bits/stdc++.h>
using namespace std;
#define fast ios::sync_with_stdio(0);cin.tie(0);
#define rep(i, n)  for (int i = 0; i < (n); i++)
#define rep1(i, n) for (int i = 1; i < (n); i++)
#define repr(i, n) for (int i = (n)-1; i >= 0; i--)
#define all(x) x.begin(), x.end()
#define sum(v) reduce(all(v))
#define pb push_back
#define int long long int
typedef  long double ld;
class bail : public exception {}; // just throw bail()
template<typename T> struct is_pair : false_type {}; template<typename T, typename U> struct is_pair<pair<T, U>> : true_type {}; template<typename T> struct is_string : false_type {}; template<> struct is_string<string> : true_type {}; template<> struct is_string<const char*> : true_type {}; template<> struct is_string<char*> : true_type {}; template<typename T, typename = void> struct is_container : false_type {}; template<typename T> struct is_container<T, void_t<decltype(declval<T>().begin()), decltype(declval<T>().end())>> : bool_constant<!is_string<T>::value> {}; template<typename T> void print(const T& x) { if constexpr (is_pair<T>::value) { cout << "("; print(x.first); cout << ", "; print(x.second); cout << ")"; } else if constexpr (is_string<T>::value) { cout << "\"" << x << "\""; } else if constexpr (is_container<T>::value) { cout << "["; bool first = true; for (const auto& elem : x) { if (!first) cout << ", "; print(elem); first = false; } cout << "]"; } else { cout << x; } }

void solve();
int32_t main() {
    fast;
    solve();
    return 0;
}

auto factor_count(int n) -> int{// returns the count
    int div = 2;
    set<int> divs;
    while(n>1) {
        while(n%div == 0) {
            divs.insert(div);
            n/=div;
        }
        div++;
    }
    return divs.size();
}

void solve() {
    int n = 1000000;
    for(int i =1;i<n; i++){
        if(factor_count(i) == 4 && factor_count(i+1) == 4 &&
           factor_count(i+2) == 4 && factor_count(i+3) == 4) {
            cout << i <<"\n";
            break;
        }
    }
}
