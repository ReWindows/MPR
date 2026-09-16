#pragma once
#include "../windissect_forwards.h"

// Reconstructed from MPR.dll by Windissect. 6 member(s).
class CUseConnection {
public:
    // Category: Ctor | Source: PDB Internal
    // Symbol: ??0CUseConnection@@QEAA@PEAUHWND__@@PEAU_NETRESOURCEW@@PEAXKKPEAEKPEAGPEAK5@Z
    CUseConnection(HWND__*, _NETRESOURCEW *, void *, unsigned long, unsigned long, unsigned char *, unsigned long, unsigned short *, unsigned long *, unsigned long *);
    // Category: Dtor | Source: PDB Internal
    // Symbol: ??1CUseConnection@@QEAA@XZ
    ~CUseConnection();
protected:
    // Category: Accessor | Source: PDB Internal
    // Symbol: ?GetResult@CUseConnection@@MEAAKXZ
    virtual unsigned long GetResult();
    // Category: Method | Source: PDB Internal
    // Symbol: ?TestProvider@CUseConnection@@MEAAKPEBU_PROVIDER@@@Z
    virtual unsigned long TestProvider(_PROVIDER const *);
    // Category: Method | Source: PDB Internal
    // Symbol: ?ValidateRoutedParameters@CUseConnection@@MEAAKPEAPEBG00@Z
    virtual unsigned long ValidateRoutedParameters(unsigned short const * *, unsigned short const * *, unsigned short const * *);
private:
    // Category: Method | Source: PDB Internal
    // Symbol: ?TestProviderWorker@CUseConnection@@AEAAKPEBU_PROVIDER@@@Z
    unsigned long TestProviderWorker(_PROVIDER const *);
};
