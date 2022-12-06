use anyhow::Result;

fn main() -> Result<()> {
    println!("{}", part_1()?);
    println!("{}", part_2()?);
    Ok(())
}

fn part_1() -> Result<i32> {
    let mut res: Vec<char> = vec![];
    INPUT.lines().for_each(|line: &str| {
        let split = line.len() / 2;
        let left = line.chars().take(split).collect::<Vec<_>>();
        let right = line.chars().skip(split).take(split).collect::<Vec<_>>();
        let mut temp_res = right
            .iter()
            .filter(|c| left.contains(c))
            .collect::<Vec<_>>();
        temp_res.dedup();
        res.extend(temp_res);
    });

    Ok(to_priority(res.into_iter()).sum())
}

fn part_2() -> Result<i32> {
    let mut res: Vec<char> = vec![];
    INPUT
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .for_each(|lines| {
            let line1 = lines[0];
            let line2 = lines[1];
            let line3 = lines[2];

            let res1 = line1.chars().filter(|c| line2.contains(*c));
            let res2 = line2
                .chars()
                .filter(|c| line3.contains(*c))
                .collect::<Vec<_>>();
            let mut res3 = res1.filter(|c| res2.contains(c)).collect::<Vec<_>>();
            res3.dedup();

            res.extend(res3)
        });

    Ok(to_priority(res.into_iter()).sum())
}

fn to_priority(iter: impl Iterator<Item = char>) -> impl Iterator<Item = i32> {
    iter.map(|c| {
        if (97..=122).contains(&(c as i32)) {
            (c as i32) - 96
        } else if (65..=90).contains(&(c as i32)) {
            (c as i32) - 38
        } else {
            0
        }
    })
}

const INPUT: &str = r#"JppMDcJPcQbqGqFb
ZStgnHtsSjGBhqFbBmsm
djzzwgHHggdnfwjtMPDPMGpPlNfpLDll
dRCtwtlCSttPtlNPNtgvPrDqmBsjGSpjBBsJsqqmrp
ZhWnZhzMMfnWWTDzBrmsmjsBccJB
TFQMrZrMfZrLZZnLQdlvNdCgdHPlCllHNF
PVddPnZWDDPqmHZzqVPzqdHdMRMJjQtvmvjvtTQQtlRtbbQl
fsNswFpChpNwfgtMvCbjTRJBbtRj
SFgfSMShLcsLgNMFhpwShFFZPHqZGZPqHdzPZHZcdzDDqd
dHffzCqSfJCCzCPvdcpzRrBDDBSBBBMtthDDrFMT
bVsZbsrgbWshBTFtDTtZTD
slWGNbNWgWwnmNnwgnlGjdJJdPHcvcrcHjqPzdqPvp
vBFBzGvGBvjFpGWcvCCvBvGPPhZbgLhMnhrPgFgPFgLLPF
dVlslfdJNVJJmQQwdNRNwLZZZrrqhbPbZndpdgMhLP
HJRHJpNJlVmftHfNQNRsHmmGCtGWjGDDTvCzcjWDStvcvT
cVQzCCCpVVjgHsNwwFRqSRSRRtNH
PWfhfPhbWWdWllPDdTvdbnvhLJLLMJFTSNwJtMRwJLqJJNMq
DnfblllPbDrbWmbdPFdvlhdgQzzZZzQgcpcsZGcGQQgcpr
hdFnnrdLnJRrZMzVlMbrwZvz
TgfjGCcfsqfqqBfRMPDVvVVvTZRPbP
qQqffcGStBSCQcsqCCBRjGqWhHFptFdnhNLmhnLWmphLLt
WHPqWhLWHBMqddCdtDRwStDSwgnw
rfCfZTmvpvvpFVfFrbvTbvcgtwgtStntggwbSJttgJtwgw
fpQFsmfFmfZcrFCMhWNQWLCzhhCM
zdqqJHDWwqNNZjQSmtdjpmfnjj
GFBcPfBLPThcTcLRbpTQQjpTQrjSnQjlvn
MFVLcsPVbPcRGfVPFRgDzJsHZNDCZNCNZsHg
WQdMhlmQMwfZlQdgdHfddvtzmbJbcsctbBJqqcvqzB
rSTrDDCSSGRCnnSGwwzJsbBvJsCCttccsvsz
VnFwDrNwPwVSDrLgjfZZdMQFMWgljQ
rJNjBLNQjvLQjFFLQJCFTvGWlRRgfGWDWlCDzlGnRRWG
tMMhShhwhbgplflwDlnnnzDH
ZcSZVMSqshhQBQqQvqQgjJ
btTtWGbvtcvHHccdNRhHdl
TDzqLPSqBzqVSldPddHhhNNlRQ
jBLDqDgZZBzJJmsbWjppmmpTWw
LLDTRRfTwZRGZfDCRTwRsVHsVFBVZVlmVFBFlFJl
QjWpQrzQcbhfMqrVVHVPFslFVpVmHH
hqhzqMfNvjvMMzhdRDdCwGDSSdnvRn
jmMdNCLjLmJPtHtSHhSVVJ
pDBqgBgCvbfGBBbCGHZhtQZtPHVSVHPV
pBnWgBpzWzBbWDWvzwpgCvqcmLNTrdLmmrnRLTrmrTdmLd
dRWWtRGBDPQctQDZ
CMfnCfFMmTTFVCmfmTLvsnMPZjccHRcPrRPsZQQZPQDRrQ
fLCffmRTCvRnmTllzVvpdpphNJggpGbJJJpdlh
cfsfbbWqqZqDScbhGDPDjTPmpVtmmPjPntFD
dCBBCRQCHNHLMQvrJJJBJLrGPMnjVtPjVMmnjPFmPmMpFn
HvRGLddgHHvqhwzbWhcgcf
DHmhhDDhbqrRhvbrHqhPbPvBwwWwTmMGTVwwBMGWVNNwVw
gSzJjnnjsscfcsZcSfWwQGdMMcWTBcTTMTMw
nsFCCFZsnSlCFBFflfflsztRPCqvvHPttRbHbhvPrrRr
SGNRLzpNpgNSNNlWFNwzqJfHfJHBtBdJhBJrHd
bQnjPTnjVdZtPPrfLP
TTnTccLnjGFcDNggFl
lBJmNzJlzmmQBlzgVVjLWRDfLjRsRLWWRS
vTMvTvtrfdhrrrPTdZwTTwtDqSSnWFWDFMMsRssjjsRqWD
ZpbvHbZddtbrdrdhcBmmpGQfBQgJNcmz
qsmhTmTVcDdcffhPhPvvzFhF
BNNlJbjWBWjtRtNljbmBBCpFzPlFPgpPfwFnlffnlPgg
JtBmmrrrRbDSrVqDqVSD
VmSgpSLgJjVDMrFrmMlfFmsG
bZWhZbzWcCwTWPRCwwwSPQzwMHnflHnGsMGnTrsflGMnlfTF
CqWhWqWCCWRRZQcLqDLpVDDLNStpgD
JwzTVzzcLzVJVVlJpVTwzGcrWhFjqsBjCjQFjWcCCjCjBF
MDSNQMtbSSnSbgbRDvnRNgvhmrqrjBCBtmCrCCWFqthhWm
bgMfNSSNZTwZQddGJZ
jjPgbFjjStjjPcSbrbtpvNrGnGDvBnMGvNDNGG
LwdZwsTdWTTmwDNCNppGBsnpBR
mWmHwZWhTTJWHBdQQWBjcFczSgjlSftjbtQPzg
fCBlrffzlCzcmfLDlfgRRnHScsHvRSQHMQvs
LthpZqtpVThZhGJtqTFvHsVnvsvMsggjHQjgQH
ZWFWGTLTWBNCWfDfmD
BpFqJrpcZZBDhDsNqMHhRG
zwlzPdmzPLhwvhMgsHnN
mPlWjlQfQjjllWtjQCWzCrBFJFTBcSNcJbbZfcZJSb
FpzNFTThBDChnnzNBCBNDzWGndgWqbJWVGqmPbJqQgQm
HQRctcvwgddVcJGd
LwRMZLjQfSRjFNzFSrhNsThS
WgRWVLWhFqgqgcgWHqLRWHVRbbbNwBmtBcNcwdwwBNwBzwBN
pSfssjDSrSvJpNJCztbJBtMNbt
TzsvGGnjGPsPjSTpDrPfPDSHhWqLgZZVHVhZZZZVQQFLnZ
sHjtPjQTtDbsfrrqWR
ZmccchvlvSvvZNMVNhvBLWRJJblWJWJlJfwHbwlL
hMMBzSMppVSzSNNmNgHjPgGPFTCpTTgTgG
rPQpCPPCQQZdcFhcZgzVJgwt
vDBSHmvHSSMlDWWmljnHBvFwzzcrwhVSJwVFtzcJJghw
bmmrsrsbsvNdCTdbPTdb
NgzBnsBNnfjgNvvfvWbtShSSFSMLJMFjjbPh
CCcRHdlQrQPLTJPLGP
HrDlRcqcDJCCDJHlcplrDJDfgfzBWWNZfvsnNqWwngZWwW
TggFVbjVTVzRwFFjjqBBqpzNztqcQqNNqN
SnZPrWndmShSZSPsnSLsJhpMQpcJtNqBClltqctC
mfWsGdnZZWmZmrLSfsrnmPggwtggRgGTVHvRwgFDjTGT
wQMZFQwppbPHPbLQJsgQNJJmBnds
qrSGvRRCvTzTDNnhNgmCgdBm
qzRrcRlGjvvzlzrvjcGqSSzMHWwVdPMWFpFMHMHWlPWMVV
BccsFzrBcsfpccccgFmQqlGNqCTLTlQnqgLlRG
MZbVSMddddSPPtdHPVJPJdTllqLNLGGTNCCRhTnHTQHh
DMwtbVZDZbSbpprcpccppwrq
bSZTdNqFjjzjqQMQ
WtRztLsWJpPLzrsMDlDQjMhVMC
pcgHRpHmWPJgLRzBJHBPPRBfvvfGNGvvFwfvSvfTmNNZfN
RjvBljWTTWTlqmBvHjGptRgccZcbPtcttbpzbn
VhDSDdJLJpSNNVVznPzzzSzcZFngZt
LsMdQNVMHsBHjqps
rSPSLTnSCClfSFCR
MwtZgwNBzzTjzZMbbmjNtwmBcRvFqQllqtQRlFRVqRqVVcvq
TsMhZsjNjBbTbBbwZLpDnHdDGdHWnpGHnh
LfdssTFBjFHSwlhzCcZZgMMfMhZZ
DmtDvrpmNStVvqpPczrZMMhZhhChCC
vJtJpQWvNVDtJFbHlJSlBbSF
PgvHLbcgRcGGHzRvgGgchBzsCZnmNZmZddrCrvdddZCZrF
JQBMjlStJrQsZFrNCZ
JSJTlqMtjptlplSSqJVDBqphwhhwHRPRggGbGzHbgzVRPH
pBsztsZdBsnWhntVnhtVTqTWNQlGGTGGFFlQFlTl
fDvLSMcbDbfMrGqFqSGwSQPzqz
zjcvLMMvffLjgggzfjjJCCLcdBtRphddBmmZBpCZZpZpmmhs
gmmSDplcPHDfHDlbVNrtCtCCNqHvTn
QGwJjzdPMzJhFLwnbvMqrTVCCMnvMv
LQQZdGjGdJhFBFQGjLGBJzzsBSWcPflllcfmDmfBsmBWPs
sMppbLDRMQbrTDTJjwcqnfnjwwnw
CgmFgSPHPzHgdmJWZZFzCzZZcVnGGnttWtVVVnccfjVqwVff
PhCChPSNgPgPJhPHhFZgsDNRspsvMNQLRvNDvpbD
pHnVnlRGVpfgpfgCpCTz
LCNQPqZCqNSSZCStzPgTmztwPfzwzT
dbSqhjhqQhNLQLFjLjGvFcRnCcsvRVvvsFnG
QTdnDTDQbCnMdbqdwSmJBljFJZhttJZMSh
HfNfLNzGLsgWBpGJthlZFBSB
ssRPHPfNsWHzZZvrvHfgLzNwnbbVVRwwRbVVcqDTddCVdD
ZhZBrJssjrNsbRtWpjwjlmlm
qTfHzGTfTGqqLGMdCCRcmmRRZcbPMccWmcRw
dVCZdDHdGHQhBVNJSsBhBF
NsplbGDbblHcbCpDlDGDPlPpJjdrVjgrvHnghvVdJrJdvvdn
tQFMZNWSmWFQRzQSwtzFmwFFJdVBghVrnBjVjBgrvVjMvrhJ
FFRmFZZRzQRmFLQSZZWQWFCPCNGcTCpGpfGfLTpblpff
dnhQHqQCnqWwNQCwCRRdJjjJVVPmVVJsJP
LccFRDgfMgLFBJVlmVVmgjrPzZ
pvfGpRpSpTnTQbvbTv
nwNlWwhWwNmJvQQdzdzZGMqDzn
rcVscPfrLcfvjTFcQDqGdZqMzMSHZHjS
LsfbbFcvVgRRblJRNR
Rmrrlmltcffhllfl
DDZMMMFZVsFsWZDSHhPPfgQbPPnQgcHctf
zGMGSBVDWCrvwjtBvm
DCZHwdDwNNGNZDZCjjtpTHLvtlgLbRLttlTL
BzPzBJffJJQgLlRgGztQ
PJffcJhJfNNSwFhGZC
blplfHbwZSfbcbwSbfVSHDHcNvdrvrWsCrvWVCRWnvNndtWW
hQzBTQLLMBWBsnsnNC
zMsgTjqTqPpZHbfp
ZLNNLtfZhRJQtpQhNRttZRqcGFcqGBzjrcqclGScFljjcj
VnvVVPMWHwgJCMvwdcGFdFrGcwzGBG
WHgPJJvWVTgnvgvTHmWCHmsNZRtNRDZpfZQffZZssR
rZgMFMVVjGbVSqZbhftLRDmCGRCNDCtm
cdddQzdWsWnQvzscnfRHmfnDNtLNChRmfH
zplDQdDzwszlDsWdPcldzMFFrFrBFgpqVBZgSgZFrF
gbzfbTvbJgbvzvTvJvJmzvjcBBQSfWDLCSBQQfLZWSCdBHHD
GhnMhsGrMNPPwnwsRPNsFCZDWHZdWBWHdrZLdCHBBC
PsnGsssGNsNRFsLGwMVNccJVmbjcjJblbzTVjlTz
RfBqNfVmPLTTTVRZMMBWjlMvBgbbMs
rHJzDwJdHzQgbMQlWMhd
HtJrpCcHflLPFfpq
SSGtmjQFStDbQbqGWJNnpZwgPsZjnJNNfZ
RdClRMhlldHdlvdThNgwfJNRspNwJpwZcN
CMBLgvvBLhrrBHTCVVLBQrWtbrDGmbtWGtzDQFGb
ZRRCqHpRdztLSqWz
hRsGjMVccGshPVDVcBmfgFFzggFgfBmWWtSd
PVjlGhcGJjGPsGMjjDVrMTpRnvRZlHnZNNQbHppTTC
QjbjWWlndRbwwwQWQdtpTVVZtzRPhHDzThhP
fFfLsvrLCrmvGlSfLSrlPHThDzTztHVtzDzGtTVh
fvrgsfccFLLLSvfMCCSWBjdMqdqNWBdBQNnWld
pjGPvvbllvqGvGjwMbpbRmgSmSwtRtShgSSWWQmW
DffTZLLzFFFTCRJMhWBzhmhgJt
CVVZLLTTsZTFCnMffMTFHffLVpGpNqGjNbNPljqVPcGNGlcv
rvJfztqQJqqrqHHwCzClTbBhDBBDrbSgphbVTrrV
FFGdNWFLGWMmLDbpRwVbgRBp
dmNGmZjMjQwtHCCw
MSGbqbqMbbGDhSSGDhLNBPNcrDPPfzfczPfrnv
rRwsrljslRgslwwgpssCjRtBNvnmPmBmccvmPnmzcP
CjVjsssJQpTCrCCgHJLMSZbhMhWGZhHG
JtBGBFGRVGVLLctRttthLFRBDQlDppljJwNQlpHHQqDHbHHN
szSZTrzdzTSMdzsmsqqQqwdbgDNqgQNjlN
SzmrlWPrlfPMsMZTPtCVCFRhhvhccLFPvL
chmbsMDMMcBnGbZBzZGL
JjjgJrJJggNgtrQSQBLjWBZlGnnB
rFNrFtFCRNVrrsHmMDcsLqMhMF
DLMwrBGgrBBDrcBcNBgWhpGhqVhhqSjqqqmjjp
JfJCZCdtlbZlHHbjWVpPmHWb
lTtZtnFQCztWFzJflZLcsrLFNDRNBcswcLMM
tsVttVCBsCcCqPqwvtqNPQjWDDWjzQQWnpJQSrWJJN
LMRdZvHGgMmZZGGLGLbhLhJpjWrSjndWJpSJrJppJnpj
RhRgLMlmgMGMFGLlTtcBvwcfFCtVvfFT
llBBmtncBglfqwsdwsjdbHwQHm
zGRhPJvFzhvFvMDGFvGPwSWWQpdpHSwSjWjHsWMw
JGFhDPrLTzhLhvFPzPjLjTzFlBqltnCqqBffcTgfnZfCfCnB
QWQGNHQBffCNDMPdRTDLPVMN
hrtlhtzZwJtwwgFgtlJthJJtZMqPVVpLmDVqRDmVPmMPVRTV
SJSSnJwJlsFFvcHSBbfLQHGL
zMNVzhNFsdNssmhlvtQvlttBlVGbTt
wHpFwjHjLFHlwtncTQlnwn
CCpjpqLCLqJggghqRmFsNRFmPszh
CTjmprmNcnmCNVQbstnstvbnQv
jqqdGhHgPRdfRRQvfLQffz
BGgqBhMdgqBHMDDqqMPwgdhSTJNjjmJWWCTCCmwTWZNpmZ
LqSDFFmdqDBDbbBHWl
dQcRpgwRQPngBWHbVWht
vdQMJQCQQvrJNqrTNsSZSmZT
cqrHcHHFNFPLLNPHLWnHHFHFjvlbZfWjSjBjZfSblhbBSbSf
VdVTGwTzTwTpMJslvjfsbbZSvnBClj
nJdGmmTpTTdmNQNNLPPmDFQL
VcjpTTtpcbThJBTTcjBvSPJzJlvPlwfJrgrgvw
dmshCNqnqdhmRsCsqCnrNgPPrPfzrNLfSwSgzl
ddQZdFnhsMttFTbDDD
hLThMTSdfMzzLzTLsFTbwtDvtsFTnttF
pZlNllPWrPCQPQlWNqjrqrjsnfDFbbtbstwswtvDnwtwjt
pCNNQBCClQrplrfHQpQdSRMSVRhJzRchRhGHSh
RZfVfRnTcPQWZcRVcRNSvljQsSSjNvvNqvss
pwbwgmqJhGlNvvzgMM
pthrhpmthmwhHHpLbbdrJmLWWRPTRZnCnPCPBnWTWTZq
nzsJJsMjGMMsQFbnNmLnmCfb
HlllPPTPlWTPDRRDRHcwwhrLLZfQmgmmQCFHvNZCgfggZQ
lPcNTwNWddGsMSttdjzj
bTbJZJDVFdqpBZTFTZJprdcsjjGszmjQszcjzDvsgccv
HHLnhMNCCPhfhCLMlPlnvgGfctscSvtSjGjztQtm
wWNQMPPMwlPHHPnnRLwWpJbrbbqbdppqJVJdZV
gpgpNnnrhwBVWFqgjqqF
ZCTsdRbCGZpZzCSGbWmtMBmWtqBBqdjjmq
LcsTSlplZrJNcQQPvH
DJDpMcqJDcDddNcJPGcJGFfnfZZmZZfRQZRNmVQVnBNn
WWThvSSHFshlSsHvgTHLlzBQfVWwmwnnfmRQWnQVRn
vljlgHjhstHbLhjLjrbPqMGJcJFJCMPrdF
pJlPMpMBrrMcnrBBMMrvdhdgFvmcFdWtmdtftg
bVVRmTRVTVVSVVVzZZqsggFHWTWtWffWFfWdtHdH
RjLQDSzmVsjRRZSZQrCMJDPwlnJJnGGrPB
GRgJtglPGlCrhQQrfW
vSjvZvZHNBjZSvwjvmvNDqhhFqhHrMhrHpfWqQfhpW
NBmwDSbZbvsNNBjmZfJzJgbVnGJbzVzPVRtL
QQZVQCdlVmdZnWmJBrLwSJRdggwdJr
DqHhPhcpvDqPFjhHjtFPssSJNfSBwBvffrgJwwJLBJbr
jpsHcptHsqtPttsjctpFhGlZwCMlWMmWGCWVQnTMVWmz
zzvnHjHWSfnvzpnfSRHdgrrsmWmhVrrwwbTrTmsrwm
NlLNGqqclqlZZCLCGGCPllJhhPJbrmpmrwVshFVsJbwr
DMLDNCZQCGLQGDtQpGtMjRdfdSRSSBtjtBjfjgjS
WBBJfjBQJjftGCbttVJptC
sdlmTHsqNsqpRtGcbWdbnR
svNMhmWHNmvmvHZWTvBQDgSSjDFPSBSFPP
mrlgqncgwHdqWWhRNtRttSvbRwwSvN
jZLCZVZzVcCTcpCDVSBBSbvvRFbfDBSDNS
VPJVpQjTZLjVLJJQGZTZGcQVghsGlmhshlqnghqrdrgldhlW
qwGDMqMFWbFbqbDwMgqgnjfnffffcTTjNnpTfBncZZ
lSdclSltlzPsJlhNNfTQPTmQhpPh
LtllsscLLLvvJlSzWGqwrDGqvbqMwMGb
bnfvnnQsVTdHQDmdNN
fwlttwrfSfLwdGZGmGND
qlPrjjplfSpjSltfttBBsvgzgbgnggBqWVvC
HHrBSmBqBHdHCFcQsc
WqtMDDnMMZhMhcsQVM
bqtbGDLLTTnRPwvGgBNmNrwvjm
RwWzWFwBcdSdMgJW
mrTjVQDQthQvjrvTmggFbbdgSJcJDFgSdg
phhFtvjNvFvTTvmTHZzlGlzZRZLBlzRp
cHlZmZmJSHZcTVGmvZVcGVHlNtbDNbtWFWdtlbCWbWtQDFdr
pwfwgRhngPjjtdWdNtFrdFwT
RTgMpfhBcMJGHqcz
dmCbpCLQVFmCRddCTFLCgZtWPNBhSNNwZWgWZvSwhN
rDzGDjJHhhwzThBS
rJrclsjjsJMJnJMLnbQQQFmRTqLVRT
TgFTGPtrWHzgCJDz
fQbfLwbbFbWjHJHWjd
LvQvwVfQvQhLLLLsmMBfLfBhqchtGFSNSFNqrSqPcqSGttSN
MsMmFgTVMMgMdFVMhdzWqCCBCWjWqBWqhzzL
nHfZDlbvcrDpcpcfDJbNJqGLbCjBmjLzJC
RnDZHwmmTwVMTsMS
NpdpdnjNCRjBnHRPpBDnhSdSwFFLFdScFFWcFWLd
stqMrMZtsQMJmMqvrtqMbstQVchPFWgFVVFVwfWJcSSWVSSV
MQrZzbvMrrrZZtZmstMtMsDRRjPHCGTjBnzpnDHTCGjG
ClGGvDMGMNhNSmFVPbpgFgmFgZPb
WhTTLWsBBWTcrPpggtQpsQFpVb
fBRTqWLjTrzDNhMvCSGDzl
jbzjttVzpbWzWVbTtzWzVwPbPPcwDDccccFNFLMFsm
rlghJZHmCHvHJvRLSNLhhhccNPwcMP
QffmBZRBGtVTBdTG
bDRqHwwRpNPnbppn
FqrlQSQJSVsQrSCmpdPNpcNCcP
jFZjrssjBhrZVhJLLGHfMHfwtqRjMqRTvD
SMMCTBzFfSRhTThCSMRSzzHnJgqGDHQgGVDnqrqnqqnqcQ
ljvjpWPbrWWtrbmDVDlmnDnqVmgJ
bNjsNPrNPtswfZzfTSTBwhSM
hNMNdssdMqdTQchqgNZzHtwmwGHHzmQZGHDH
JrbJvJrvLrPjrPCVCjRBLPbzfRFZmzwHgFHmGHmwFwzFZR
rpbrvVjPCBbCJrCSLSPsTlphNncMglcNTMWMql
fGWGHbrllCCWWllFNPQSZvdPSvdZTffZ
tqssVjJMJWzWVvSvWS
qtwMwDWjnRRwssWjngwjjnhhDcCLhhCFHlcmcbCHFmDC
rFTdFjdRDTTlDWCqvhwLhwZdLS
QzfJfnfsbsJHMnNmHhVpCZwSQSWqSqVQhS
nJnczsHzNMmBJnbnbNwnfzJfcDTltllRTgPlFlgPrTDjPRGl
ZLCGDvvJlvGChSPZWPSsZWdRRN
rQccBwcccnHmQggnVLPLWpgVWPpWzSRs
fmwTfTHnMBTfJDbfftJLvhlL"#;
