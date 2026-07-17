                PART 'Sine/Cosine LUT generator'
;                     =========================

SC_N            equ $00000100
SC_STEPS_PI     equ SC_N>>1
SC_COS_OFFS     equ SC_STEPS_PI>>1
SC_FIXPOINT     equ 14
SC_ONE          equ 1<<SC_FIXPOINT
SC_STEP_R       equ $00003ffb   ; SC_ONE * cos( Pi / STEPS_PI )
SC_STEP_I       equ $00000192   ; SC_ONE * sin( Pi / STEPS_PI )


sc_lutz:        lea     sc_lut(pc),a0
                lea     2*(SC_COS_OFFS)(a0),a1
                lea     2*(SC_STEPS_PI)(a0),a2
                lea     2*(SC_STEPS_PI+SC_COS_OFFS)(a0),a3
                lea     2*(SC_STEPS_PI*2)(a0),a4
                move.w  #SC_ONE,d0
                moveq   #0,d1
                moveq   #SC_FIXPOINT,d4
                move.w  #SC_COS_OFFS-1,d5
sc_loop:        movem.w d0-d1,-(sp)
                move.w  d1,(a0)+
                move.w  d1,(a4)+
                move.w  d0,(a1)+
                neg.w   d0
                neg.w   d1
                move.w  d1,(a2)+
                move.w  d0,(a3)+
                movem.w (sp)+,d2-d3
                muls    #-SC_STEP_R,d0
                muls    #-SC_STEP_I,d3
                add.l   d3,d0
                lsr.l   d4,d0
                muls    #SC_STEP_I,d2
                muls    #-SC_STEP_R,d1
                add.l   d2,d1
                lsr.l   d4,d1
                dbra    d5,sc_loop
                rts

                endpart

;-------------------------------------------------------------------------------

                bss

;-------------------------------------------------------------------------------
                PART 'Sine/Cosine LUT (BSS)'
;                     =====================

                even
sc_lut:         ds.w SC_N+SC_COS_OFFS
                endpart

