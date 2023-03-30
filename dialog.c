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

extern ConstStringPtr hello_rust(void);
extern void ConvertCtoF(unsigned char *, unsigned char *);
extern void ConvertFtoC(unsigned char *, unsigned char *);
extern void present_error(OSStatus);
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
    kNewToot = 1,
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

#define CELCIUS_ITEM 7
#define FAREN_ITEM 8

enum
{
    kPostButton = 1
};

void UpdateMenus()
{
    // MenuRef m = GetMenuHandle(kMenuFile);
    // WindowRef w = FrontWindow();
    //
    // m = GetMenuHandle(kMenuFile);
    // if(w && (w == aboutWindow || GetWindowKind(w) < 0))
    //     EnableItem(m,kItemClose);
    // else
    //     DisableItem(m,kItemClose);
    //
    // m = GetMenuHandle(kMenuEdit);
    //
    // bool enableEditMenu = (w && GetWindowKind(w) < 0);
    // // Desk accessory in front: Enable edit menu items
    // // Application window or nothing in front, disable edit menu
    //
    // for(short i : {1,3,4,5,6})
    //     SetItemEnabled(m,i,enableEditMenu);
    //
    // m = GetMenuHandle(kMenuConnection);
    // SetItemEnabled(m, 1, portsAvailable[(int)Port::macTCP]);
    // CheckMenuItem(m, 1, gPrefs.port == Port::macTCP);
    // SetItemEnabled(m, 2, portsAvailable[(int)Port::openTptTCP]);
    // CheckMenuItem(m, 2, gPrefs.port == Port::openTptTCP);
    // SetItemEnabled(m, 3, portsAvailable[(int)Port::modemPort]);
    // CheckMenuItem(m, 3, gPrefs.port == Port::modemPort);
    // SetItemEnabled(m, 4, portsAvailable[(int)Port::printerPort]);
    // CheckMenuItem(m, 4, gPrefs.port == Port::printerPort);
    // SetItemEnabled(m, 5, portsAvailable[(int)Port::sharedFiles]);
    // CheckMenuItem(m, 5, gPrefs.port == Port::sharedFiles);
    // for(int i = 7; i < kItemChooseFolder; i++)
    // {
    //     Str255 str;
    //     long baud;
    //     GetMenuItemText(m, i, str);
    //     StringToNum(str, &baud);
    //     CheckMenuItem(m, i, baud == gPrefs.baud);
    //     SetItemEnabled(m, i, gPrefs.port == Port::modemPort || gPrefs.port == Port::printerPort);
    // }
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
// #if !TARGET_API_MAC_CARBON
//         else
//         {
//             GetMenuItemText(GetMenu(128), menuItem, str);
//             OpenDeskAcc(str);
//         }
// #endif
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
    // else if(menuID == kMenuConnection)
    // {
    //     switch(menuItem)
    //     {
    //         case 1:
    //             gPrefs.port = Port::macTCP;
    //             break;
    //         case 2:
    //             gPrefs.port = Port::openTptTCP;
    //             break;
    //         case 3:
    //             gPrefs.port = Port::modemPort;
    //             break;
    //         case 4:
    //             gPrefs.port = Port::printerPort;
    //             break;
    //         case 5:
    //             gPrefs.port = Port::sharedFiles;
    //             break;
    //         case kItemChooseFolder:
    //             ChooseSharedDirectory();
    //             UnloadSeg((void*) &ChooseSharedDirectory);
    //             break;
    //         default:
    //             GetMenuItemText(GetMenuHandle(menuID), menuItem, str);
    //             StringToNum(str, &gPrefs.baud);
    //     }
    //     ConnectionChanged();
    // }
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
    // SelectDialogItemText(dlg,CELCIUS_ITEM,0,32767);
    //
    // ConstStr255Param param1 = hello_rust();
    //
    // ParamText(param1, "\p", "\p", "\p");
    //
    // DialogItemType type;
    // Handle itemH;
    // Rect box;
    //
    // Str255 celciusStr;
    // Str255 farenheitStr;
    //
    // GetDialogItem(dlg, 2, &type, &itemH, &box);
    // GetDialogItem(dlg, 2, &type, &itemH, &box);
    // SetDialogItem(dlg, 2, type, (Handle) NewUserItemUPP(&ButtonFrameProc), &box);

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

            // if (item == CELCIUS_ITEM || item == FAREN_ITEM) {
            //     // Update text values
            //     GetDialogItem(dlg, CELCIUS_ITEM, &type, &itemH, &box);
            //     GetDialogItemText(itemH, celciusStr);
            //     GetDialogItem(dlg, FAREN_ITEM, &type, &itemH, &box);
            //     GetDialogItemText(itemH, farenheitStr);
            // }
            //
            switch (item) {
                //     // Typed in Celcius field, update Farenheit
                //     case CELCIUS_ITEM:
                //         ConvertCtoF(celciusStr, farenheitStr);
                //         // Update the text of dialog item
                //         GetDialogItem(dlg, FAREN_ITEM, &type, &itemH, &box); // TODO: Avoid re-getting this?
                //         SetDialogItemText(itemH, farenheitStr);
                //         break;
                //     case FAREN_ITEM:
                //         ConvertFtoC(farenheitStr, celciusStr);
                //         // Update the text of dialog item
                //         GetDialogItem(dlg, CELCIUS_ITEM, &type, &itemH, &box); // TODO: Avoid re-getting this
                //         SetDialogItemText(itemH, celciusStr);
                //         break;
                case kPostButton:
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
