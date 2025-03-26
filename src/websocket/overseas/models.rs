use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverseasOrderNotificationData {
    /// 고객 ID
    pub cust_id: String,
    /// 계좌번호
    pub acnt_no: String,
    /// 주문번호
    pub oder_no: String,
    /// 원주문번호
    pub ooder_no: String,
    /// 매도매수구분 (01:매도 02:매수 03:전매도 04:환매수)
    pub seln_byov_cls: String,
    /// 정정구분 (0:정상 1:정정 2:취소)
    pub rctf_cls: String,
    /// 주문종류2 (1:시장가 2:지정자 6:단주시장가 7:단주지정가 A:MOO B:LOO C:MOC D:LOC)
    pub oder_kind2: String,
    /// 주식 단축 종목코드
    pub stck_shrn_iscd: String,
    /// 체결수량 (주문통보의 경우 주문수량, 체결통보인 경우 체결수량)
    pub cntg_qty: String,
    /// 체결단가 (주문통보 시 주문단가, 체결통보 시 체결단가)
    pub cntg_unpr: String,
    /// 주식 체결 시간
    pub stck_cntg_hour: String,
    /// 거부여부 (0:정상 1:거부)
    pub rfus_yn: String,
    /// 체결여부 (1:주문,정정,취소,거부 2:체결)
    pub cntg_yn: String,
    /// 접수여부 (1:주문접수 2:확인 3:취소(FOK/IOC))
    pub acpt_yn: String,
    /// 지점번호
    pub brnc_no: String,
    /// 주문 수량 (주문통보인 경우 미출력, 체결통보인 경우 주문수량 출력)
    pub oder_qty: String,
    /// 계좌명
    pub acnt_name: String,
    /// 체결종목명
    pub cntg_isnm: String,
    /// 해외종목구분 (4:홍콩(HKD) 5:상해B(USD) 6:NASDAQ 7:NYSE 8:AMEX 9:OTCB C:홍콩(CNY) A:상해A(CNY) B:심천B(HKD) D:도쿄 E:하노이 F:호치민)
    pub oder_cond: String,
    /// 담보유형코드 (0:현금 15:해외주식담보대출)
    pub debt_gb: String,
    /// 담보대출일자 (YYYYMMDD)
    pub debt_date: String,
}


pub trait RealtimeData: Sized {
    /// 구분자(^)로 나뉜 문자열에서 구조체 생성
    fn from_delimited_string(text: &str) -> Option<Self>;
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverseasQuoteData {
    /// 실시간종목코드
    pub rsym: String,
    /// 종목코드
    pub symb: String,
    /// 소수점자리수
    pub zdiv: String,
    /// 현지일자
    pub xymd: String,
    /// 현지시간
    pub xhms: String,
    /// 한국일자
    pub kymd: String,
    /// 한국시간
    pub khms: String,
    /// 매수총잔량
    pub bvol: String,
    /// 매도총잔량
    pub avol: String,
    /// 매수총잔량대비
    pub bdvl: String,
    /// 매도총잔량대비
    pub advl: String,
    /// 매수호가1
    pub pbid1: String,
    /// 매도호가1
    pub pask1: String,
    /// 매수잔량1
    pub vbid1: String,
    /// 매도잔량1
    pub vask1: String,
    /// 매수잔량대비1
    pub dbid1: String,
    /// 매도잔량대비1
    pub dask1: String,
}
/// 해외 실시간 데이터 구조체
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverseasRealtimeData {
    /// 실시간종목코드
    pub rsym: String,
    /// 종목코드
    pub symb: String,
    /// 수수점자리수
    pub zdiv: String,
    /// 현지영업일자 (8자리)
    pub tymd: String,
    /// 현지일자 (6자리)
    pub xymd: String,
    /// 현지시간 (6자리)
    pub xhms: String,
    /// 한국일자 (6자리)
    pub kymd: String,
    /// 한국시간 (6자리)
    pub khms: String,
    /// 시가
    pub open: String,
    /// 고가
    pub high: String,
    /// 저가
    pub low: String,
    /// 현재가
    pub last: String,
    /// 대비구분 (상승/하락 등)
    pub sign: String,
    /// 전일대비
    pub diff: String,
    /// 등락율
    pub rate: String,
    /// 매수호가
    pub pbid: String,
    /// 매도호가
    pub pask: String,
    /// 매수잔량
    pub vbid: String,
    /// 매도잔량
    pub vask: String,
    /// 체결량
    pub evol: String,
    /// 거래량
    pub tvol: String,
    /// 거래대금
    pub tamt: String,
    /// 매도체결량
    pub bivl: String,
    /// 매수체결량
    pub asvl: String,
    /// 체결강도
    pub strn: String,
    /// 시장구분 (1:장중, 2:장전, 3:장후)
    pub mtyp: String,
}

/// 해외 실시간 호가 데이터 구조체
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverseasOrderbookData {
    /// 실시간종목코드
    pub rsym: String,
    /// 종목코드
    pub symb: String,
    /// 소숫점자리수
    pub zdiv: String,
    /// 현지일자
    pub xymd: String,
    /// 현지시간
    pub xhms: String,
    /// 한국일자
    pub kymd: String,
    /// 한국시간
    pub khms: String,
    /// 매수총잔량
    pub bvol: String,
    /// 매도총잔량
    pub avol: String,
    /// 매수총잔량대비
    pub bdvl: String,
    /// 매도총잔량대비
    pub advl: String,

    // 1단계 호가 정보
    /// 매수호가1
    pub pbid1: String,
    /// 매도호가1
    pub pask1: String,
    /// 매수잔량1
    pub vbid1: String,
    /// 매도잔량1
    pub vask1: String,
    /// 매수잔량대비1
    pub dbid1: String,
    /// 매도잔량대비1
    pub dask1: String,

    // 2단계 호가 정보
    /// 매수호가2
    pub pbid2: String,
    /// 매도호가2
    pub pask2: String,
    /// 매수잔량2
    pub vbid2: String,
    /// 매도잔량2
    pub vask2: String,
    /// 매수잔량대비2
    pub dbid2: String,
    /// 매도잔량대비2
    pub dask2: String,

    // 3단계 호가 정보
    /// 매수호가3
    pub pbid3: String,
    /// 매도호가3
    pub pask3: String,
    /// 매수잔량3
    pub vbid3: String,
    /// 매도잔량3
    pub vask3: String,
    /// 매수잔량대비3
    pub dbid3: String,
    /// 매도잔량대비3
    pub dask3: String,

    // 4단계 호가 정보
    /// 매수호가4
    pub pbid4: String,
    /// 매도호가4
    pub pask4: String,
    /// 매수잔량4
    pub vbid4: String,
    /// 매도잔량4
    pub vask4: String,
    /// 매수잔량대비4
    pub dbid4: String,
    /// 매도잔량대비4
    pub dask4: String,

    // 5단계 호가 정보
    /// 매수호가5
    pub pbid5: String,
    /// 매도호가5
    pub pask5: String,
    /// 매수잔량5
    pub vbid5: String,
    /// 매도잔량5
    pub vask5: String,
    /// 매수잔량대비5
    pub dbid5: String,
    /// 매도잔량대비5
    pub dask5: String,

    // 6단계 호가 정보
    /// 매수호가6
    pub pbid6: String,
    /// 매도호가6
    pub pask6: String,
    /// 매수잔량6
    pub vbid6: String,
    /// 매도잔량6
    pub vask6: String,
    /// 매수잔량대비6
    pub dbid6: String,
    /// 매도잔량대비6
    pub dask6: String,

    // 7단계 호가 정보
    /// 매수호가7
    pub pbid7: String,
    /// 매도호가7
    pub pask7: String,
    /// 매수잔량7
    pub vbid7: String,
    /// 매도잔량7
    pub vask7: String,
    /// 매수잔량대비7
    pub dbid7: String,
    /// 매도잔량대비7
    pub dask7: String,

    // 8단계 호가 정보
    /// 매수호가8
    pub pbid8: String,
    /// 매도호가8
    pub pask8: String,
    /// 매수잔량8
    pub vbid8: String,
    /// 매도잔량8
    pub vask8: String,
    /// 매수잔량대비8
    pub dbid8: String,
    /// 매도잔량대비8
    pub dask8: String,

    // 9단계 호가 정보
    /// 매수호가9
    pub pbid9: String,
    /// 매도호가9
    pub pask9: String,
    /// 매수잔량9
    pub vbid9: String,
    /// 매도잔량9
    pub vask9: String,
    /// 매수잔량대비9
    pub dbid9: String,
    /// 매도잔량대비9
    pub dask9: String,

    // 10단계 호가 정보
    /// 매수호가10
    pub pbid10: String,
    /// 매도호가10
    pub pask10: String,
    /// 매수잔량10
    pub vbid10: String,
    /// 매도잔량10
    pub vask10: String,
    /// 매수잔량대비10
    pub dbid10: String,
    /// 매도잔량대비10
    pub dask10: String,
}
// OverseasOrderbookData에 대한 RealtimeData 구현

impl RealtimeData for OverseasOrderbookData {
    fn from_delimited_string(text: &str) -> Option<Self> {
        let fields: Vec<&str> = text.split('^').collect();

        if fields.len() < 71 {
            // 호가 정보는 필드가 많습니다
            return None;
        }

        Some(Self {
            rsym: fields[0].to_string(),
            symb: fields[1].to_string(),
            zdiv: fields[2].to_string(),
            xymd: fields[3].to_string(),
            xhms: fields[4].to_string(),
            kymd: fields[5].to_string(),
            khms: fields[6].to_string(),
            bvol: fields[7].to_string(),
            avol: fields[8].to_string(),
            bdvl: fields[9].to_string(),
            advl: fields[10].to_string(),

            // 1단계 호가
            pbid1: fields[11].to_string(),
            pask1: fields[12].to_string(),
            vbid1: fields[13].to_string(),
            vask1: fields[14].to_string(),
            dbid1: fields[15].to_string(),
            dask1: fields[16].to_string(),

            // 2단계 호가
            pbid2: fields[17].to_string(),
            pask2: fields[18].to_string(),
            vbid2: fields[19].to_string(),
            vask2: fields[20].to_string(),
            dbid2: fields[21].to_string(),
            dask2: fields[22].to_string(),

            // 3단계 호가
            pbid3: fields[23].to_string(),
            pask3: fields[24].to_string(),
            vbid3: fields[25].to_string(),
            vask3: fields[26].to_string(),
            dbid3: fields[27].to_string(),
            dask3: fields[28].to_string(),

            // 4단계 호가
            pbid4: fields[29].to_string(),
            pask4: fields[30].to_string(),
            vbid4: fields[31].to_string(),
            vask4: fields[32].to_string(),
            dbid4: fields[33].to_string(),
            dask4: fields[34].to_string(),

            // 5단계 호가
            pbid5: fields[35].to_string(),
            pask5: fields[36].to_string(),
            vbid5: fields[37].to_string(),
            vask5: fields[38].to_string(),
            dbid5: fields[39].to_string(),
            dask5: fields[40].to_string(),

            // 6단계 호가
            pbid6: fields[41].to_string(),
            pask6: fields[42].to_string(),
            vbid6: fields[43].to_string(),
            vask6: fields[44].to_string(),
            dbid6: fields[45].to_string(),
            dask6: fields[46].to_string(),

            // 7단계 호가
            pbid7: fields[47].to_string(),
            pask7: fields[48].to_string(),
            vbid7: fields[49].to_string(),
            vask7: fields[50].to_string(),
            dbid7: fields[51].to_string(),
            dask7: fields[52].to_string(),

            // 8단계 호가
            pbid8: fields[53].to_string(),
            pask8: fields[54].to_string(),
            vbid8: fields[55].to_string(),
            vask8: fields[56].to_string(),
            dbid8: fields[57].to_string(),
            dask8: fields[58].to_string(),

            // 9단계 호가
            pbid9: fields[59].to_string(),
            pask9: fields[60].to_string(),
            vbid9: fields[61].to_string(),
            vask9: fields[62].to_string(),
            dbid9: fields[63].to_string(),
            dask9: fields[64].to_string(),

            // 10단계 호가
            pbid10: fields[65].to_string(),
            pask10: fields[66].to_string(),
            vbid10: fields[67].to_string(),
            vask10: fields[68].to_string(),
            dbid10: fields[69].to_string(),
            dask10: fields[70].to_string(),
        })
    }
}

// OverseasOrderNotificationData에 대한 RealtimeData 구현
impl RealtimeData for OverseasOrderNotificationData {
    fn from_delimited_string(text: &str) -> Option<Self> {
        let fields: Vec<&str> = text.split('^').collect();

        if fields.len() < 22 {
            return None;
        }

        Some(Self {
            cust_id: fields[0].to_string(),
            acnt_no: fields[1].to_string(),
            oder_no: fields[2].to_string(),
            ooder_no: fields[3].to_string(),
            seln_byov_cls: fields[4].to_string(),
            rctf_cls: fields[5].to_string(),
            oder_kind2: fields[6].to_string(),
            stck_shrn_iscd: fields[7].to_string(),
            cntg_qty: fields[8].to_string(),
            cntg_unpr: fields[9].to_string(),
            stck_cntg_hour: fields[10].to_string(),
            rfus_yn: fields[11].to_string(),
            cntg_yn: fields[12].to_string(),
            acpt_yn: fields[13].to_string(),
            brnc_no: fields[14].to_string(),
            oder_qty: fields[15].to_string(),
            acnt_name: fields[16].to_string(),
            cntg_isnm: fields[17].to_string(),
            oder_cond: fields[18].to_string(),
            debt_gb: fields[19].to_string(),
            debt_date: fields[20].to_string(),
        })
    }
}

// OverseasQuoteData에 대한 RealtimeData 구현도 필요합니다
impl RealtimeData for OverseasQuoteData {
    fn from_delimited_string(text: &str) -> Option<Self> {
        let fields: Vec<&str> = text.split('^').collect();

        if fields.len() < 17 {
            return None;
        }

        Some(Self {
            rsym: fields[0].to_string(),
            symb: fields[1].to_string(),
            zdiv: fields[2].to_string(),
            xymd: fields[3].to_string(),
            xhms: fields[4].to_string(),
            kymd: fields[5].to_string(),
            khms: fields[6].to_string(),
            bvol: fields[7].to_string(),
            avol: fields[8].to_string(),
            bdvl: fields[9].to_string(),
            advl: fields[10].to_string(),
            pbid1: fields[11].to_string(),
            pask1: fields[12].to_string(),
            vbid1: fields[13].to_string(),
            vask1: fields[14].to_string(),
            dbid1: fields[15].to_string(),
            dask1: fields[16].to_string(),
        })
    }
}
impl RealtimeData for OverseasRealtimeData {
    fn from_delimited_string(text: &str) -> Option<Self> {
        let fields: Vec<&str> = text.split('^').collect();

        if fields.len() < 26 {
            return None;
        }

        Some(Self {
            rsym: fields[0].to_string(),
            symb: fields[1].to_string(),
            zdiv: fields[2].to_string(),
            tymd: fields[3].to_string(),
            xymd: fields[4].to_string(),
            xhms: fields[5].to_string(),
            kymd: fields[6].to_string(),
            khms: fields[7].to_string(),
            open: fields[8].to_string(),
            high: fields[9].to_string(),
            low: fields[10].to_string(),
            last: fields[11].to_string(),
            sign: fields[12].to_string(),
            diff: fields[13].to_string(),
            rate: fields[14].to_string(),
            pbid: fields[15].to_string(),
            pask: fields[16].to_string(),
            vbid: fields[17].to_string(),
            vask: fields[18].to_string(),
            evol: fields[19].to_string(),
            tvol: fields[20].to_string(),
            tamt: fields[21].to_string(),
            bivl: fields[22].to_string(),
            asvl: fields[23].to_string(),
            strn: fields[24].to_string(),
            mtyp: fields[25].to_string(),
        })
    }
}
