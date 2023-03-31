/*
    Copyright 2015 Wolfgang Thaller.

    This file is part of Retro68.

    Retro68 is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    Retro68 is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with Retro68.  If not, see <http://www.gnu.org/licenses/>.
*/

#include <Quickdraw.h>
#include <Dialogs.h>
#include <Fonts.h>

#ifndef TARGET_API_MAC_CARBON
/* NOTE: this is checking whether the Dialogs.h we use *knows* about Carbon,
         not whether we are actually compiling for Cabon.
         If Dialogs.h is older, we add a define to be able to use the new name
         for NewUserItemUPP, which used to be NewUserItemProc. */

#define NewUserItemUPP NewUserItemProc
#endif

extern void do_request(void);

Boolean gQuitting = false;

enum
{
    kMenuApple = 128,
    kMenuFile,
    kMenuEdit
};

// Apple menu
enum
{
    kItemAbout = 1,
};

// FIle menu
enum
{
    kNewLocation = 1,
    kItemClose = 3,
    kItemQuit = 5,
};

pascal void ButtonFrameProc(DialogRef dlg, DialogItemIndex itemNo)
{
    DialogItemType type;
    Handle itemH;
    Rect box;

    GetDialogItem(dlg, 1, &type, &itemH, &box);
    InsetRect(&box, -4, -4);
    PenSize(3, 3);
    FrameRoundRect(&box, 16, 16);
}

enum
{
    kGetWeatherButton = 1
};

void UpdateMenus()
{
    // TODO
}

void ShowAboutBox(void)
{
    // TODO
}

void DoMenuCommand(long menuCommand)
{
    Str255 str;
    WindowRef w;
    short menuID = menuCommand >> 16;
    short menuItem = menuCommand & 0xFFFF;
    if (menuID == kMenuApple) {
        if (menuItem == kItemAbout)
            ShowAboutBox();
    }
    else if (menuID == kMenuFile) {
        switch (menuItem) {
//             case kItemClose:
//                 w = FrontWindow();
//                 if(w)
//                 {
// #if !TARGET_API_MAC_CARBON
//                     if(GetWindowKind(w) < 0)
//                         CloseDeskAcc(GetWindowKind(w));
//                     else
// #endif
//                     if(w == aboutWindow)
//                     {
//                         DisposeWindow(w);
//                         aboutWindow = nullptr;
//                     }
//                 }
//                 break;
//
            case kItemQuit:
                gQuitting = true;
                break;
        }
    }
    else if (menuID == kMenuEdit) {
// #if !TARGET_API_MAC_CARBON
//         if(!SystemEdit(menuItem - 1))
// #endif
//         {
//             // edit command not handled by desk accessory
//         }
    }
    HiliteMenu(0);
}

int main(void)
{
#if !TARGET_API_MAC_CARBON
    InitGraf(&qd.thePort);
    InitFonts();
    InitWindows();
    InitMenus();
    TEInit();
    InitDialogs(NULL);
#endif
    SetMenuBar(GetNewMBar(128));
    DrawMenuBar();

    DialogRef dlg = GetNewDialog(129, 0, (WindowPtr) -1);
    InitCursor();

    DialogItemIndex item;
    DialogRef theDialog = NULL;
    WindowRef win = NULL;
    EventRecord ev;
    do {
        SystemTask();
        GetNextEvent(everyEvent, &ev);
        // Inside Macintosh: If your modeless dialog contains any textEdit items, you must
        // call IsDialogEvent (and then DialogSelect) even if GetNextEvent returns FALSE;
        // otherwise your dialog won't receive null events and the caret won' blink.
        if (IsDialogEvent(&ev) && DialogSelect(&ev, &theDialog, &item)) {
            // ModalDialog(NULL, &item);

            switch (item) {
                case kGetWeatherButton:
                    do_request();
                    break;
            }
        }
        else {
            switch (ev.what) {
                case mouseDown:
                    switch (FindWindow(ev.where, &win)) {
                        case inDrag:
                            DragWindow(win, ev.where, &qd.screenBits.bounds);
                            break;
                        case inMenuBar:
                            UpdateMenus();
                            DoMenuCommand(MenuSelect(ev.where));
                            break;
                    }
            }

        }
    }
    while (!gQuitting);

    FlushEvents(everyEvent, -1);
    return 0;
}
