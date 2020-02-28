/*------------------------------------------------------------------------
 *  Copyright 2007-2009 (c) Jeff Brown <spadix@users.sourceforge.net>
 *
 *  This file is part of the ZBar Bar Code Reader.
 *
 *  The ZBar Bar Code Reader is free software; you can redistribute it
 *  and/or modify it under the terms of the GNU Lesser Public License as
 *  published by the Free Software Foundation; either version 2.1 of
 *  the License, or (at your option) any later version.
 *
 *  The ZBar Bar Code Reader is distributed in the hope that it will be
 *  useful, but WITHOUT ANY WARRANTY; without even the implied warranty
 *  of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU Lesser Public License for more details.
 *
 *  You should have received a copy of the GNU Lesser Public License
 *  along with the ZBar Bar Code Reader; if not, write to the Free
 *  Software Foundation, Inc., 51 Franklin St, Fifth Floor,
 *  Boston, MA  02110-1301  USA
 *
 *  http://sourceforge.net/projects/zbar
 *------------------------------------------------------------------------*/

#include "refcnt.h"

#include <config.h>
#include <assert.h>

#if defined(_WIN32)
# include <windows.h>

int _zbar_refcnt (refcnt_t *cnt,
                                int delta)
{
    int rc = -1;
    if(delta > 0)
        while(delta--)
            rc = InterlockedIncrement(cnt);
    else if(delta < 0)
        while(delta++)
            rc = InterlockedDecrement(cnt);
    assert(rc >= 0);
    return(rc);
}

#elif defined(TARGET_OS_MAC)
# include <libkern/OSAtomic.h>

int _zbar_refcnt (refcnt_t *cnt,
                                int delta)
{
    int rc = OSAtomicAdd32Barrier(delta, cnt);
    assert(rc >= 0);
    return(rc);
}

#elif defined(HAVE_LIBPTHREAD)
# include <pthread.h>

extern pthread_mutex_t _zbar_reflock;

int _zbar_refcnt (refcnt_t *cnt,
                                int delta)
{
    pthread_mutex_lock(&_zbar_reflock);
    int rc = (*cnt += delta);
    pthread_mutex_unlock(&_zbar_reflock);
    assert(rc >= 0);
    return(rc);
}


#else

int _zbar_refcnt (refcnt_t *cnt,
                                int delta)
{
    int rc = (*cnt += delta);
    assert(rc >= 0);
    return(rc);
}

#endif

#if !defined(_WIN32) && !defined(TARGET_OS_MAC) && defined(HAVE_LIBPTHREAD)

pthread_once_t initialized = PTHREAD_ONCE_INIT;
pthread_mutex_t _zbar_reflock;

static void initialize (void)
{
    pthread_mutex_init(&_zbar_reflock, NULL);
}

void _zbar_refcnt_init ()
{
    pthread_once(&initialized, initialize);
}


#else

void _zbar_refcnt_init ()
{
}

#endif
