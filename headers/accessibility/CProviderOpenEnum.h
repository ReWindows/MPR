#pragma once
#include "../windissect_forwards.h"

// Reconstructed from MPR.dll by Windissect. 3 member(s).
class CProviderOpenEnum {
protected:
    // Category: Accessor | Source: PDB Internal
    // Symbol: ?GetResult@CProviderOpenEnum@@MEAAKXZ
    virtual unsigned long GetResult();
    // Category: Method | Source: PDB Internal
    // Symbol: ?TestProvider@CProviderOpenEnum@@MEAAKPEBU_PROVIDER@@@Z
    virtual unsigned long TestProvider(_PROVIDER const *);
    // Category: Method | Source: PDB Internal
    // Symbol: ?ValidateRoutedParameters@CProviderOpenEnum@@MEAAKPEAPEBG00@Z
    virtual unsigned long ValidateRoutedParameters(unsigned short const * *, unsigned short const * *, unsigned short const * *);
};
