#pragma once
#include "../windissect_forwards.h"

// Reconstructed from MPR.dll by Windissect. 2 member(s).
namespace CRoutedOperation {
class CPathCache {
public:
    // Category: Method | Source: PDB Internal
    // Symbol: ?FindEntry@CPathCache@CRoutedOperation@@QEAAPEAU_PROVIDER@@PEBU_UNICODE_STRING@@@Z
    _PROVIDER * FindEntry(_UNICODE_STRING const *);
    // Category: Method | Source: PDB Internal
    // Symbol: ?Flush@CPathCache@CRoutedOperation@@QEAAXXZ
    void Flush();
};
} // namespace CRoutedOperation
