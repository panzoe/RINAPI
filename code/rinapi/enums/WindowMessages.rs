use super::super::prelude::WindowMessage;

/**
    Window Messages
    WinUser.h:1719 -> 2223
**/

pub static                           Null : WindowMessage = 0x0000;
pub static                         Create : WindowMessage = 0x0001;
pub static                        Destroy : WindowMessage = 0x0002;
pub static                           Move : WindowMessage = 0x0003;
pub static                           Size : WindowMessage = 0x0005;
pub static                       Activate : WindowMessage = 0x0006;
pub static                       SetFocus : WindowMessage = 0x0007;
pub static                      KillFocus : WindowMessage = 0x0008;
pub static                         Enable : WindowMessage = 0x000A;
pub static                      SetRedraw : WindowMessage = 0x000B;
pub static                        SetText : WindowMessage = 0x000C;
pub static                        GetText : WindowMessage = 0x000D;
pub static                  GetTextLength : WindowMessage = 0x000E;
pub static                          Paint : WindowMessage = 0x000F;
pub static                          Close : WindowMessage = 0x0010;
pub static                QUERYENDSESSION : WindowMessage = 0x0011;
pub static                      QUERYOPEN : WindowMessage = 0x0013;
pub static                     ENDSESSION : WindowMessage = 0x0016;
pub static                           Quit : WindowMessage = 0x0012;
pub static                     ERASEBKGND : WindowMessage = 0x0014;
pub static                 SYSCOLORCHANGE : WindowMessage = 0x0015;
pub static                     SHOWWINDOW : WindowMessage = 0x0018;
pub static                   WININICHANGE : WindowMessage = 0x001A;
pub static                  SETTINGCHANGE : WindowMessage = WININICHANGE;
pub static                  DEVMODECHANGE : WindowMessage = 0x001B;
pub static                    ACTIVATEAPP : WindowMessage = 0x001C;
pub static                     FONTCHANGE : WindowMessage = 0x001D;
pub static                     TIMECHANGE : WindowMessage = 0x001E;
pub static                     CANCELMODE : WindowMessage = 0x001F;
pub static                      SETCURSOR : WindowMessage = 0x0020;
pub static                  MOUSEACTIVATE : WindowMessage = 0x0021;
pub static                  CHILDACTIVATE : WindowMessage = 0x0022;
pub static                      QUEUESYNC : WindowMessage = 0x0023;
pub static                  GETMINMAXINFO : WindowMessage = 0x0024;
pub static                      PAINTICON : WindowMessage = 0x0026;
pub static                 ICONERASEBKGND : WindowMessage = 0x0027;
pub static                     NEXTDLGCTL : WindowMessage = 0x0028;
pub static                  SPOOLERSTATUS : WindowMessage = 0x002A;
pub static                       DRAWITEM : WindowMessage = 0x002B;
pub static                    MEASUREITEM : WindowMessage = 0x002C;
pub static                     DELETEITEM : WindowMessage = 0x002D;
pub static                     VKEYTOITEM : WindowMessage = 0x002E;
pub static                     CHARTOITEM : WindowMessage = 0x002F;
pub static                        SETFONT : WindowMessage = 0x0030;
pub static                        GETFONT : WindowMessage = 0x0031;
pub static                      SETHOTKEY : WindowMessage = 0x0032;
pub static                      GETHOTKEY : WindowMessage = 0x0033;
pub static                  QUERYDRAGICON : WindowMessage = 0x0037;
pub static                    COMPAREITEM : WindowMessage = 0x0039;
pub static                      GETOBJECT : WindowMessage = 0x003D;
pub static                     COMPACTING : WindowMessage = 0x0041;
pub static                     COMMNOTIFY : WindowMessage = 0x0044;
pub static              WINDOWPOSCHANGING : WindowMessage = 0x0046;
pub static               WINDOWPOSCHANGED : WindowMessage = 0x0047;
pub static                          POWER : WindowMessage = 0x0048;
pub static                       COPYDATA : WindowMessage = 0x004A;
pub static                  CANCELJOURNAL : WindowMessage = 0x004B;
pub static                         NOTIFY : WindowMessage = 0x004E;
pub static         INPUTLANGCHANGEREQUEST : WindowMessage = 0x0050;
pub static                INPUTLANGCHANGE : WindowMessage = 0x0051;
pub static                          TCARD : WindowMessage = 0x0052;
pub static                           HELP : WindowMessage = 0x0053;
pub static                    USERCHANGED : WindowMessage = 0x0054;
pub static                   NOTIFYFORMAT : WindowMessage = 0x0055;
pub static                    CONTEXTMENU : WindowMessage = 0x007B;
pub static                  STYLECHANGING : WindowMessage = 0x007C;
pub static                   STYLECHANGED : WindowMessage = 0x007D;
pub static                  DISPLAYCHANGE : WindowMessage = 0x007E;
pub static                        GETICON : WindowMessage = 0x007F;
pub static                        SETICON : WindowMessage = 0x0080;
pub static                       NCCREATE : WindowMessage = 0x0081;
pub static                      NCDESTROY : WindowMessage = 0x0082;
pub static                     NCCALCSIZE : WindowMessage = 0x0083;
pub static                      NCHITTEST : WindowMessage = 0x0084;
pub static                        NCPAINT : WindowMessage = 0x0085;
pub static                     NCACTIVATE : WindowMessage = 0x0086;
pub static                     GETDLGCODE : WindowMessage = 0x0087;
pub static                      SYNCPAINT : WindowMessage = 0x0088;
pub static                    NCMOUSEMOVE : WindowMessage = 0x00A0;
pub static                  NCLBUTTONDOWN : WindowMessage = 0x00A1;
pub static                    NCLBUTTONUP : WindowMessage = 0x00A2;
pub static                NCLBUTTONDBLCLK : WindowMessage = 0x00A3;
pub static                  NCRBUTTONDOWN : WindowMessage = 0x00A4;
pub static                    NCRBUTTONUP : WindowMessage = 0x00A5;
pub static                NCRBUTTONDBLCLK : WindowMessage = 0x00A6;
pub static                  NCMBUTTONDOWN : WindowMessage = 0x00A7;
pub static                    NCMBUTTONUP : WindowMessage = 0x00A8;
pub static                NCMBUTTONDBLCLK : WindowMessage = 0x00A9;
pub static                  NCXBUTTONDOWN : WindowMessage = 0x00AB;
pub static                    NCXBUTTONUP : WindowMessage = 0x00AC;
pub static                NCXBUTTONDBLCLK : WindowMessage = 0x00AD;
pub static            INPUT_DEVICE_CHANGE : WindowMessage = 0x00FE;
pub static                          INPUT : WindowMessage = 0x00FF;
pub static                       KEYFIRST : WindowMessage = 0x0100;
pub static                        KEYDOWN : WindowMessage = 0x0100;
pub static                          KEYUP : WindowMessage = 0x0101;
pub static                           CHAR : WindowMessage = 0x0102;
pub static                       DEADCHAR : WindowMessage = 0x0103;
pub static                     SYSKEYDOWN : WindowMessage = 0x0104;
pub static                       SYSKEYUP : WindowMessage = 0x0105;
pub static                        SYSCHAR : WindowMessage = 0x0106;
pub static                    SYSDEADCHAR : WindowMessage = 0x0107;
pub static                        UNICHAR : WindowMessage = 0x0109;
pub static                        KEYLAST : WindowMessage = 0x0109;
pub static           IME_STARTCOMPOSITION : WindowMessage = 0x010D;
pub static             IME_ENDCOMPOSITION : WindowMessage = 0x010E;
pub static                IME_COMPOSITION : WindowMessage = 0x010F;
pub static                    IME_KEYLAST : WindowMessage = 0x010F;
pub static                     INITDIALOG : WindowMessage = 0x0110;
pub static                        COMMAND : WindowMessage = 0x0111;
pub static                     SYSCOMMAND : WindowMessage = 0x0112;
pub static                          TIMER : WindowMessage = 0x0113;
pub static                        HSCROLL : WindowMessage = 0x0114;
pub static                        VSCROLL : WindowMessage = 0x0115;
pub static                       INITMENU : WindowMessage = 0x0116;
pub static                  INITMENUPOPUP : WindowMessage = 0x0117;
pub static                        GESTURE : WindowMessage = 0x0119;
pub static                  GESTURENOTIFY : WindowMessage = 0x011A;
pub static                     MENUSELECT : WindowMessage = 0x011F;
pub static                       MENUCHAR : WindowMessage = 0x0120;
pub static                      ENTERIDLE : WindowMessage = 0x0121;
pub static                  MENURBUTTONUP : WindowMessage = 0x0122;
pub static                       MENUDRAG : WindowMessage = 0x0123;
pub static                  MENUGETOBJECT : WindowMessage = 0x0124;
pub static                UNINITMENUPOPUP : WindowMessage = 0x0125;
pub static                    MENUCOMMAND : WindowMessage = 0x0126;
pub static                  CHANGEUISTATE : WindowMessage = 0x0127;
pub static                  UPDATEUISTATE : WindowMessage = 0x0128;
pub static                   QUERYUISTATE : WindowMessage = 0x0129;
pub static                 CTLCOLORMSGBOX : WindowMessage = 0x0132;
pub static                   CTLCOLOREDIT : WindowMessage = 0x0133;
pub static                CTLCOLORLISTBOX : WindowMessage = 0x0134;
pub static                    CTLCOLORBTN : WindowMessage = 0x0135;
pub static                    CTLCOLORDLG : WindowMessage = 0x0136;
pub static              CTLCOLORSCROLLBAR : WindowMessage = 0x0137;
pub static                 CTLCOLORSTATIC : WindowMessage = 0x0138;
pub static                     MOUSEFIRST : WindowMessage = 0x0200;
pub static                      MOUSEMOVE : WindowMessage = 0x0200;
pub static                    LBUTTONDOWN : WindowMessage = 0x0201;
pub static                      LBUTTONUP : WindowMessage = 0x0202;
pub static                  LBUTTONDBLCLK : WindowMessage = 0x0203;
pub static                    RBUTTONDOWN : WindowMessage = 0x0204;
pub static                      RBUTTONUP : WindowMessage = 0x0205;
pub static                  RBUTTONDBLCLK : WindowMessage = 0x0206;
pub static                    MBUTTONDOWN : WindowMessage = 0x0207;
pub static                      MBUTTONUP : WindowMessage = 0x0208;
pub static                  MBUTTONDBLCLK : WindowMessage = 0x0209;
pub static                     MOUSEWHEEL : WindowMessage = 0x020A;
pub static                    XBUTTONDOWN : WindowMessage = 0x020B;
pub static                      XBUTTONUP : WindowMessage = 0x020C;
pub static                  XBUTTONDBLCLK : WindowMessage = 0x020D;
pub static                    MOUSEHWHEEL : WindowMessage = 0x020E;
pub static                      MOUSELAST : WindowMessage = 0x020E;
pub static                   PARENTNOTIFY : WindowMessage = 0x0210;
pub static                  ENTERMENULOOP : WindowMessage = 0x0211;
pub static                   EXITMENULOOP : WindowMessage = 0x0212;
pub static                       NEXTMENU : WindowMessage = 0x0213;
pub static                         SIZING : WindowMessage = 0x0214;
pub static                 CAPTURECHANGED : WindowMessage = 0x0215;
pub static                         MOVING : WindowMessage = 0x0216;
pub static                 POWERBROADCAST : WindowMessage = 0x0218;
pub static                   DEVICECHANGE : WindowMessage = 0x0219;
pub static                      MDICREATE : WindowMessage = 0x0220;
pub static                     MDIDESTROY : WindowMessage = 0x0221;
pub static                    MDIACTIVATE : WindowMessage = 0x0222;
pub static                     MDIRESTORE : WindowMessage = 0x0223;
pub static                        MDINEXT : WindowMessage = 0x0224;
pub static                    MDIMAXIMIZE : WindowMessage = 0x0225;
pub static                        MDITILE : WindowMessage = 0x0226;
pub static                     MDICASCADE : WindowMessage = 0x0227;
pub static                 MDIICONARRANGE : WindowMessage = 0x0228;
pub static                   MDIGETACTIVE : WindowMessage = 0x0229;
pub static                     MDISETMENU : WindowMessage = 0x0230;
pub static                  ENTERSIZEMOVE : WindowMessage = 0x0231;
pub static                   EXITSIZEMOVE : WindowMessage = 0x0232;
pub static                      DROPFILES : WindowMessage = 0x0233;
pub static                 MDIREFRESHMENU : WindowMessage = 0x0234;
pub static                          TOUCH : WindowMessage = 0x0240;
pub static                 IME_SETCONTEXT : WindowMessage = 0x0281;
pub static                     IME_NOTIFY : WindowMessage = 0x0282;
pub static                    IME_CONTROL : WindowMessage = 0x0283;
pub static            IME_COMPOSITIONFULL : WindowMessage = 0x0284;
pub static                     IME_SELECT : WindowMessage = 0x0285;
pub static                       IME_CHAR : WindowMessage = 0x0286;
pub static                    IME_REQUEST : WindowMessage = 0x0288;
pub static                    IME_KEYDOWN : WindowMessage = 0x0290;
pub static                      IME_KEYUP : WindowMessage = 0x0291;
pub static                     MOUSEHOVER : WindowMessage = 0x02A1;
pub static                     MOUSELEAVE : WindowMessage = 0x02A3;
pub static                   NCMOUSEHOVER : WindowMessage = 0x02A0;
pub static                   NCMOUSELEAVE : WindowMessage = 0x02A2;
pub static              WTSSESSION_CHANGE : WindowMessage = 0x02B1;
pub static                   TABLET_FIRST : WindowMessage = 0x02c0;
pub static                    TABLET_LAST : WindowMessage = 0x02df;
pub static                            CUT : WindowMessage = 0x0300;
pub static                           COPY : WindowMessage = 0x0301;
pub static                          PASTE : WindowMessage = 0x0302;
pub static                          CLEAR : WindowMessage = 0x0303;
pub static                           UNDO : WindowMessage = 0x0304;
pub static                   RENDERFORMAT : WindowMessage = 0x0305;
pub static               RENDERALLFORMATS : WindowMessage = 0x0306;
pub static               DESTROYCLIPBOARD : WindowMessage = 0x0307;
pub static                  DRAWCLIPBOARD : WindowMessage = 0x0308;
pub static                 PAINTCLIPBOARD : WindowMessage = 0x0309;
pub static               VSCROLLCLIPBOARD : WindowMessage = 0x030A;
pub static                  SIZECLIPBOARD : WindowMessage = 0x030B;
pub static                ASKCBFORMATNAME : WindowMessage = 0x030C;
pub static                  CHANGECBCHAIN : WindowMessage = 0x030D;
pub static               HSCROLLCLIPBOARD : WindowMessage = 0x030E;
pub static                QUERYNEWPALETTE : WindowMessage = 0x030F;
pub static              PALETTEISCHANGING : WindowMessage = 0x0310;
pub static                 PALETTECHANGED : WindowMessage = 0x0311;
pub static                         HOTKEY : WindowMessage = 0x0312;
pub static                          PRINT : WindowMessage = 0x0317;
pub static                    PRINTCLIENT : WindowMessage = 0x0318;
pub static                     APPCOMMAND : WindowMessage = 0x0319;
pub static                   THEMECHANGED : WindowMessage = 0x031A;
pub static                CLIPBOARDUPDATE : WindowMessage = 0x031D;
pub static          DWMCOMPOSITIONCHANGED : WindowMessage = 0x031E;
pub static          DWMNCRENDERINGCHANGED : WindowMessage = 0x031F;
pub static    DWMCOLORIZATIONCOLORCHANGED : WindowMessage = 0x0320;
pub static       DWMWINDOWMAXIMIZEDCHANGE : WindowMessage = 0x0321;
pub static         DWMSENDICONICTHUMBNAIL : WindowMessage = 0x0323;
pub static DWMSENDICONICLIVEPREVIEWBITMAP : WindowMessage = 0x0326;
pub static              GETTITLEBARINFOEX : WindowMessage = 0x033F;
pub static                  HANDHELDFIRST : WindowMessage = 0x0358;
pub static                   HANDHELDLAST : WindowMessage = 0x035F;
pub static                       AFXFIRST : WindowMessage = 0x0360;
pub static                        AFXLAST : WindowMessage = 0x037F;
pub static                    PENWINFIRST : WindowMessage = 0x0380;
pub static                     PENWINLAST : WindowMessage = 0x038F;
pub static                            APP : WindowMessage = 0x8000;