#pragma once
#include "../windissect_forwards.h"

// Reconstructed from MPR.dll by Windissect. 2 member(s).
class CGetConnectionPerformance {
protected:
    // Category: Method | Source: PDB Internal
    // Symbol: ?TestProvider@CGetConnectionPerformance@@MEAAKPEBU_PROVIDER@@@Z
    virtual unsigned long TestProvider(_PROVIDER const *);
    // Category: Method | Source: PDB Internal
    // Symbol: ?ValidateRoutedParameters@CGetConnectionPerformance@@MEAAKPEAPEBG00@Z
    virtual unsigned long ValidateRoutedParameters(unsigned short const * *, unsigned short const * *, unsigned short const * *);
};
