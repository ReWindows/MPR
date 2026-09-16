#pragma once
#include "../windissect_forwards.h"

// Reconstructed from MPR.dll by Windissect. 3 member(s).
class CRoutedOperation {
public:
    class CPathCache;
public:
    // Category: Method | Source: PDB Internal
    // Symbol: ?Perform@CRoutedOperation@@QEAAKH@Z
    unsigned long Perform(int);
protected:
    // Category: Accessor | Source: PDB Internal
    // Symbol: ?GetResult@CRoutedOperation@@MEAAKXZ
    virtual unsigned long GetResult();
    // Category: Method | Source: PDB Internal
    // Symbol: ?ValidateParameters@CRoutedOperation@@MEAAKXZ
    virtual unsigned long ValidateParameters();
};
