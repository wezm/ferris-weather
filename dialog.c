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

pascal void ButtonFrameProc(DialogRef dlg, DialogItemIndex itemNo)
{
    DialogItemType type;
    Handle itemH;
    Rect box;

    GetDialogItem(dlg, 1, &type, &itemH, &box);
    InsetRect(&box, -4, -4);
    PenSize(3,3);
    FrameRoundRect(&box,16,16);
}

#define CELCIUS_ITEM 7
#define FAREN_ITEM 8

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
    DialogPtr dlg = GetNewDialog(128,0,(WindowPtr)-1);
    InitCursor();
    SelectDialogItemText(dlg,CELCIUS_ITEM,0,32767);

    ConstStr255Param param1 = hello_rust();

    ParamText(param1, "\p", "\p", "\p");

    DialogItemType type;
    Handle itemH;
    Rect box;

    Str255 celciusStr;
    Str255 farenheitStr;

    GetDialogItem(dlg, 2, &type, &itemH, &box);
    GetDialogItem(dlg, 2, &type, &itemH, &box);
    SetDialogItem(dlg, 2, type, (Handle) NewUserItemUPP(&ButtonFrameProc), &box);

    short item;
    do {
        ModalDialog(NULL, &item);

        if (item == CELCIUS_ITEM || item == FAREN_ITEM) {
            // Update text values
            GetDialogItem(dlg, CELCIUS_ITEM, &type, &itemH, &box);
            GetDialogItemText(itemH, celciusStr);
            GetDialogItem(dlg, FAREN_ITEM, &type, &itemH, &box);
            GetDialogItemText(itemH, farenheitStr);
        }

        switch (item) {
          // Typed in Celcius field, update Farenheit
          case CELCIUS_ITEM:
            ConvertCtoF(celciusStr, farenheitStr);
            // Update the text of dialog item
            GetDialogItem(dlg, FAREN_ITEM, &type, &itemH, &box); // TODO: Avoid re-getting this?
            SetDialogItemText(itemH, farenheitStr);
            break;
          case FAREN_ITEM:
            ConvertFtoC(farenheitStr, celciusStr);
            // Update the text of dialog item
            GetDialogItem(dlg, CELCIUS_ITEM, &type, &itemH, &box); // TODO: Avoid re-getting this
            SetDialogItemText(itemH, celciusStr);
            break;
        }
    } while(item != 1);

    FlushEvents(everyEvent, -1);
    return 0;
}
