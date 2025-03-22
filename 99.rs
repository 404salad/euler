#![allow(unused)]
// a^b / c^d > or < 1 for comparing powers
// used logicical math // just write down a
fn main() {
    let nums: [f64; 2000] = [
        519432.0, 525806.0, 632382.0, 518061.0, 78864.0, 613712.0, 466580.0, 530130.0, 780495.0,
        510032.0, 525895.0, 525320.0, 15991.0, 714883.0, 960290.0, 502358.0, 760018.0, 511029.0,
        166800.0, 575487.0, 210884.0, 564478.0, 555151.0, 523163.0, 681146.0, 515199.0, 563395.0,
        522587.0, 738250.0, 512126.0, 923525.0, 503780.0, 595148.0, 520429.0, 177108.0, 572629.0,
        750923.0, 511482.0, 440902.0, 532446.0, 881418.0, 505504.0, 422489.0, 534197.0, 979858.0,
        501616.0, 685893.0, 514935.0, 747477.0, 511661.0, 167214.0, 575367.0, 234140.0, 559696.0,
        940238.0, 503122.0, 728969.0, 512609.0, 232083.0, 560102.0, 900971.0, 504694.0, 688801.0,
        514772.0, 189664.0, 569402.0, 891022.0, 505104.0, 445689.0, 531996.0, 119570.0, 591871.0,
        821453.0, 508118.0, 371084.0, 539600.0, 911745.0, 504251.0, 623655.0, 518600.0, 144361.0,
        582486.0, 352442.0, 541775.0, 420726.0, 534367.0, 295298.0, 549387.0, 6530.0, 787777.0,
        468397.0, 529976.0, 672336.0, 515696.0, 431861.0, 533289.0, 84228.0, 610150.0, 805376.0,
        508857.0, 444409.0, 532117.0, 33833.0, 663511.0, 381850.0, 538396.0, 402931.0, 536157.0,
        92901.0, 604930.0, 304825.0, 548004.0, 731917.0, 512452.0, 753734.0, 511344.0, 51894.0,
        637373.0, 151578.0, 580103.0, 295075.0, 549421.0, 303590.0, 548183.0, 333594.0, 544123.0,
        683952.0, 515042.0, 60090.0, 628880.0, 951420.0, 502692.0, 28335.0, 674991.0, 714940.0,
        513349.0, 343858.0, 542826.0, 549279.0, 523586.0, 804571.0, 508887.0, 260653.0, 554881.0,
        291399.0, 549966.0, 402342.0, 536213.0, 408889.0, 535550.0, 40328.0, 652524.0, 375856.0,
        539061.0, 768907.0, 510590.0, 165993.0, 575715.0, 976327.0, 501755.0, 898500.0, 504795.0,
        360404.0, 540830.0, 478714.0, 529095.0, 694144.0, 514472.0, 488726.0, 528258.0, 841380.0,
        507226.0, 328012.0, 544839.0, 22389.0, 690868.0, 604053.0, 519852.0, 329514.0, 544641.0,
        772965.0, 510390.0, 492798.0, 527927.0, 30125.0, 670983.0, 895603.0, 504906.0, 450785.0,
        531539.0, 840237.0, 507276.0, 380711.0, 538522.0, 63577.0, 625673.0, 76801.0, 615157.0,
        502694.0, 527123.0, 597706.0, 520257.0, 310484.0, 547206.0, 944468.0, 502959.0, 121283.0,
        591152.0, 451131.0, 531507.0, 566499.0, 522367.0, 425373.0, 533918.0, 40240.0, 652665.0,
        39130.0, 654392.0, 714926.0, 513355.0, 469219.0, 529903.0, 806929.0, 508783.0, 287970.0,
        550487.0, 92189.0, 605332.0, 103841.0, 599094.0, 671839.0, 515725.0, 452048.0, 531421.0,
        987837.0, 501323.0, 935192.0, 503321.0, 88585.0, 607450.0, 613883.0, 519216.0, 144551.0,
        582413.0, 647359.0, 517155.0, 213902.0, 563816.0, 184120.0, 570789.0, 258126.0, 555322.0,
        502546.0, 527130.0, 407655.0, 535678.0, 401528.0, 536306.0, 477490.0, 529193.0, 841085.0,
        507237.0, 732831.0, 512408.0, 833000.0, 507595.0, 904694.0, 504542.0, 581435.0, 521348.0,
        455545.0, 531110.0, 873558.0, 505829.0, 94916.0, 603796.0, 720176.0, 513068.0, 545034.0,
        523891.0, 246348.0, 557409.0, 556452.0, 523079.0, 832015.0, 507634.0, 173663.0, 573564.0,
        502634.0, 527125.0, 250732.0, 556611.0, 569786.0, 522139.0, 216919.0, 563178.0, 521815.0,
        525623.0, 92304.0, 605270.0, 164446.0, 576167.0, 753413.0, 511364.0, 11410.0, 740712.0,
        448845.0, 531712.0, 925072.0, 503725.0, 564888.0, 522477.0, 7062.0, 780812.0, 641155.0,
        517535.0, 738878.0, 512100.0, 636204.0, 517828.0, 372540.0, 539436.0, 443162.0, 532237.0,
        571192.0, 522042.0, 655350.0, 516680.0, 299741.0, 548735.0, 581914.0, 521307.0, 965471.0,
        502156.0, 513441.0, 526277.0, 808682.0, 508700.0, 237589.0, 559034.0, 543300.0, 524025.0,
        804712.0, 508889.0, 247511.0, 557192.0, 543486.0, 524008.0, 504383.0, 526992.0, 326529.0,
        545039.0, 792493.0, 509458.0, 86033.0, 609017.0, 126554.0, 589005.0, 579379.0, 521481.0,
        948026.0, 502823.0, 404777.0, 535969.0, 265767.0, 554022.0, 266876.0, 553840.0, 46631.0,
        643714.0, 492397.0, 527958.0, 856106.0, 506581.0, 795757.0, 509305.0, 748946.0, 511584.0,
        294694.0, 549480.0, 409781.0, 535463.0, 775887.0, 510253.0, 543747.0, 523991.0, 210592.0,
        564536.0, 517119.0, 525990.0, 520253.0, 525751.0, 247926.0, 557124.0, 592141.0, 520626.0,
        346580.0, 542492.0, 544969.0, 523902.0, 506501.0, 526817.0, 244520.0, 557738.0, 144745.0,
        582349.0, 69274.0, 620858.0, 292620.0, 549784.0, 926027.0, 503687.0, 736320.0, 512225.0,
        515528.0, 526113.0, 407549.0, 535688.0, 848089.0, 506927.0, 24141.0, 685711.0, 9224.0,
        757964.0, 980684.0, 501586.0, 175259.0, 573121.0, 489160.0, 528216.0, 878970.0, 505604.0,
        969546.0, 502002.0, 525207.0, 525365.0, 690461.0, 514675.0, 156510.0, 578551.0, 659778.0,
        516426.0, 468739.0, 529945.0, 765252.0, 510770.0, 76703.0, 615230.0, 165151.0, 575959.0,
        29779.0, 671736.0, 928865.0, 503569.0, 577538.0, 521605.0, 927555.0, 503618.0, 185377.0,
        570477.0, 974756.0, 501809.0, 800130.0, 509093.0, 217016.0, 563153.0, 365709.0, 540216.0,
        774508.0, 510320.0, 588716.0, 520851.0, 631673.0, 518104.0, 954076.0, 502590.0, 777828.0,
        510161.0, 990659.0, 501222.0, 597799.0, 520254.0, 786905.0, 509727.0, 512547.0, 526348.0,
        756449.0, 511212.0, 869787.0, 505988.0, 653747.0, 516779.0, 84623.0, 609900.0, 839698.0,
        507295.0, 30159.0, 670909.0, 797275.0, 509234.0, 678136.0, 515373.0, 897144.0, 504851.0,
        989554.0, 501263.0, 413292.0, 535106.0, 55297.0, 633667.0, 788650.0, 509637.0, 486748.0,
        528417.0, 150724.0, 580377.0, 56434.0, 632490.0, 77207.0, 614869.0, 588631.0, 520859.0,
        611619.0, 519367.0, 100006.0, 601055.0, 528924.0, 525093.0, 190225.0, 569257.0, 851155.0,
        506789.0, 682593.0, 515114.0, 613043.0, 519275.0, 514673.0, 526183.0, 877634.0, 505655.0,
        878905.0, 505602.0, 1926.0, 914951.0, 613245.0, 519259.0, 152481.0, 579816.0, 841774.0,
        507203.0, 71060.0, 619442.0, 865335.0, 506175.0, 90244.0, 606469.0, 302156.0, 548388.0,
        399059.0, 536557.0, 478465.0, 529113.0, 558601.0, 522925.0, 69132.0, 620966.0, 267663.0,
        553700.0, 988276.0, 501310.0, 378354.0, 538787.0, 529909.0, 525014.0, 161733.0, 576968.0,
        758541.0, 511109.0, 823425.0, 508024.0, 149821.0, 580667.0, 269258.0, 553438.0, 481152.0,
        528891.0, 120871.0, 591322.0, 972322.0, 501901.0, 981350.0, 501567.0, 676129.0, 515483.0,
        950860.0, 502717.0, 119000.0, 592114.0, 392252.0, 537272.0, 191618.0, 568919.0, 946699.0,
        502874.0, 289555.0, 550247.0, 799322.0, 509139.0, 703886.0, 513942.0, 194812.0, 568143.0,
        261823.0, 554685.0, 203052.0, 566221.0, 217330.0, 563093.0, 734748.0, 512313.0, 391759.0,
        537328.0, 807052.0, 508777.0, 564467.0, 522510.0, 59186.0, 629748.0, 113447.0, 594545.0,
        518063.0, 525916.0, 905944.0, 504492.0, 613922.0, 519213.0, 439093.0, 532607.0, 445946.0,
        531981.0, 230530.0, 560399.0, 297887.0, 549007.0, 459029.0, 530797.0, 403692.0, 536075.0,
        855118.0, 506616.0, 963127.0, 502245.0, 841711.0, 507208.0, 407411.0, 535699.0, 924729.0,
        503735.0, 914823.0, 504132.0, 333725.0, 544101.0, 176345.0, 572832.0, 912507.0, 504225.0,
        411273.0, 535308.0, 259774.0, 555036.0, 632853.0, 518038.0, 119723.0, 591801.0, 163902.0,
        576321.0, 22691.0, 689944.0, 402427.0, 536212.0, 175769.0, 572988.0, 837260.0, 507402.0,
        603432.0, 519893.0, 313679.0, 546767.0, 538165.0, 524394.0, 549026.0, 523608.0, 61083.0,
        627945.0, 898345.0, 504798.0, 992556.0, 501153.0, 369999.0, 539727.0, 32847.0, 665404.0,
        891292.0, 505088.0, 152715.0, 579732.0, 824104.0, 507997.0, 234057.0, 559711.0, 730507.0,
        512532.0, 960529.0, 502340.0, 388395.0, 537687.0, 958170.0, 502437.0, 57105.0, 631806.0,
        186025.0, 570311.0, 993043.0, 501133.0, 576770.0, 521664.0, 215319.0, 563513.0, 927342.0,
        503628.0, 521353.0, 525666.0, 39563.0, 653705.0, 752516.0, 511408.0, 110755.0, 595770.0,
        309749.0, 547305.0, 374379.0, 539224.0, 919184.0, 503952.0, 990652.0, 501226.0, 647780.0,
        517135.0, 187177.0, 570017.0, 168938.0, 574877.0, 649558.0, 517023.0, 278126.0, 552016.0,
        162039.0, 576868.0, 658512.0, 516499.0, 498115.0, 527486.0, 896583.0, 504868.0, 561170.0,
        522740.0, 747772.0, 511647.0, 775093.0, 510294.0, 652081.0, 516882.0, 724905.0, 512824.0,
        499707.0, 527365.0, 47388.0, 642755.0, 646668.0, 517204.0, 571700.0, 522007.0, 180430.0,
        571747.0, 710015.0, 513617.0, 435522.0, 532941.0, 98137.0, 602041.0, 759176.0, 511070.0,
        486124.0, 528467.0, 526942.0, 525236.0, 878921.0, 505604.0, 408313.0, 535602.0, 926980.0,
        503640.0, 882353.0, 505459.0, 566887.0, 522345.0, 3326.0, 853312.0, 911981.0, 504248.0,
        416309.0, 534800.0, 392991.0, 537199.0, 622829.0, 518651.0, 148647.0, 581055.0, 496483.0,
        527624.0, 666314.0, 516044.0, 48562.0, 641293.0, 672618.0, 515684.0, 443676.0, 532187.0,
        274065.0, 552661.0, 265386.0, 554079.0, 347668.0, 542358.0, 31816.0, 667448.0, 181575.0,
        571446.0, 961289.0, 502320.0, 365689.0, 540214.0, 987950.0, 501317.0, 932299.0, 503440.0,
        27388.0, 677243.0, 746701.0, 511701.0, 492258.0, 527969.0, 147823.0, 581323.0, 57918.0,
        630985.0, 838849.0, 507333.0, 678038.0, 515375.0, 27852.0, 676130.0, 850241.0, 506828.0,
        818403.0, 508253.0, 131717.0, 587014.0, 850216.0, 506834.0, 904848.0, 504529.0, 189758.0,
        569380.0, 392845.0, 537217.0, 470876.0, 529761.0, 925353.0, 503711.0, 285431.0, 550877.0,
        454098.0, 531234.0, 823910.0, 508003.0, 318493.0, 546112.0, 766067.0, 510730.0, 261277.0,
        554775.0, 421530.0, 534289.0, 694130.0, 514478.0, 120439.0, 591498.0, 213308.0, 563949.0,
        854063.0, 506662.0, 365255.0, 540263.0, 165437.0, 575872.0, 662240.0, 516281.0, 289970.0,
        550181.0, 847977.0, 506933.0, 546083.0, 523816.0, 413252.0, 535113.0, 975829.0, 501767.0,
        361540.0, 540701.0, 235522.0, 559435.0, 224643.0, 561577.0, 736350.0, 512229.0, 328303.0,
        544808.0, 35022.0, 661330.0, 307838.0, 547578.0, 474366.0, 529458.0, 873755.0, 505819.0,
        73978.0, 617220.0, 827387.0, 507845.0, 670830.0, 515791.0, 326511.0, 545034.0, 309909.0,
        547285.0, 400970.0, 536363.0, 884827.0, 505352.0, 718307.0, 513175.0, 28462.0, 674699.0,
        599384.0, 520150.0, 253565.0, 556111.0, 284009.0, 551093.0, 343403.0, 542876.0, 446557.0,
        531921.0, 992372.0, 501160.0, 961601.0, 502308.0, 696629.0, 514342.0, 919537.0, 503945.0,
        894709.0, 504944.0, 892201.0, 505051.0, 358160.0, 541097.0, 448503.0, 531745.0, 832156.0,
        507636.0, 920045.0, 503924.0, 926137.0, 503675.0, 416754.0, 534757.0, 254422.0, 555966.0,
        92498.0, 605151.0, 826833.0, 507873.0, 660716.0, 516371.0, 689335.0, 514746.0, 160045.0,
        577467.0, 814642.0, 508425.0, 969939.0, 501993.0, 242856.0, 558047.0, 76302.0, 615517.0,
        472083.0, 529653.0, 587101.0, 520964.0, 99066.0, 601543.0, 498005.0, 527503.0, 709800.0,
        513624.0, 708000.0, 513716.0, 20171.0, 698134.0, 285020.0, 550936.0, 266564.0, 553891.0,
        981563.0, 501557.0, 846502.0, 506991.0, 334.0, 1190800.0, 209268.0, 564829.0, 9844.0,
        752610.0, 996519.0, 501007.0, 410059.0, 535426.0, 432931.0, 533188.0, 848012.0, 506929.0,
        966803.0, 502110.0, 983434.0, 501486.0, 160700.0, 577267.0, 504374.0, 526989.0, 832061.0,
        507640.0, 392825.0, 537214.0, 443842.0, 532165.0, 440352.0, 532492.0, 745125.0, 511776.0,
        13718.0, 726392.0, 661753.0, 516312.0, 70500.0, 619875.0, 436952.0, 532814.0, 424724.0,
        533973.0, 21954.0, 692224.0, 262490.0, 554567.0, 716622.0, 513264.0, 907584.0, 504425.0,
        60086.0, 628882.0, 837123.0, 507412.0, 971345.0, 501940.0, 947162.0, 502855.0, 139920.0,
        584021.0, 68330.0, 621624.0, 666452.0, 516038.0, 731446.0, 512481.0, 953350.0, 502619.0,
        183157.0, 571042.0, 845400.0, 507045.0, 651548.0, 516910.0, 20399.0, 697344.0, 861779.0,
        506331.0, 629771.0, 518229.0, 801706.0, 509026.0, 189207.0, 569512.0, 737501.0, 512168.0,
        719272.0, 513115.0, 479285.0, 529045.0, 136046.0, 585401.0, 896746.0, 504860.0, 891735.0,
        505067.0, 684771.0, 514999.0, 865309.0, 506184.0, 379066.0, 538702.0, 503117.0, 527090.0,
        621780.0, 518717.0, 209518.0, 564775.0, 677135.0, 515423.0, 987500.0, 501340.0, 197049.0,
        567613.0, 329315.0, 544673.0, 236756.0, 559196.0, 357092.0, 541226.0, 520440.0, 525733.0,
        213471.0, 563911.0, 956852.0, 502490.0, 702223.0, 514032.0, 404943.0, 535955.0, 178880.0,
        572152.0, 689477.0, 514734.0, 691351.0, 514630.0, 866669.0, 506128.0, 370561.0, 539656.0,
        739805.0, 512051.0, 71060.0, 619441.0, 624861.0, 518534.0, 261660.0, 554714.0, 366137.0,
        540160.0, 166054.0, 575698.0, 601878.0, 519990.0, 153445.0, 579501.0, 279899.0, 551729.0,
        379166.0, 538691.0, 423209.0, 534125.0, 675310.0, 515526.0, 145641.0, 582050.0, 691353.0,
        514627.0, 917468.0, 504026.0, 284778.0, 550976.0, 81040.0, 612235.0, 161699.0, 576978.0,
        616394.0, 519057.0, 767490.0, 510661.0, 156896.0, 578431.0, 427408.0, 533714.0, 254849.0,
        555884.0, 737217.0, 512182.0, 897133.0, 504851.0, 203815.0, 566051.0, 270822.0, 553189.0,
        135854.0, 585475.0, 778805.0, 510111.0, 784373.0, 509847.0, 305426.0, 547921.0, 733418.0,
        512375.0, 732087.0, 512448.0, 540668.0, 524215.0, 702898.0, 513996.0, 628057.0, 518328.0,
        640280.0, 517587.0, 422405.0, 534204.0, 10604.0, 746569.0, 746038.0, 511733.0, 839808.0,
        507293.0, 457417.0, 530938.0, 479030.0, 529064.0, 341758.0, 543090.0, 620223.0, 518824.0,
        251661.0, 556451.0, 561790.0, 522696.0, 497733.0, 527521.0, 724201.0, 512863.0, 489217.0,
        528217.0, 415623.0, 534867.0, 624610.0, 518548.0, 847541.0, 506953.0, 432295.0, 533249.0,
        400391.0, 536421.0, 961158.0, 502319.0, 139173.0, 584284.0, 421225.0, 534315.0, 579083.0,
        521501.0, 74274.0, 617000.0, 701142.0, 514087.0, 374465.0, 539219.0, 217814.0, 562985.0,
        358972.0, 540995.0, 88629.0, 607424.0, 288597.0, 550389.0, 285819.0, 550812.0, 538400.0,
        524385.0, 809930.0, 508645.0, 738326.0, 512126.0, 955461.0, 502535.0, 163829.0, 576343.0,
        826475.0, 507891.0, 376488.0, 538987.0, 102234.0, 599905.0, 114650.0, 594002.0, 52815.0,
        636341.0, 434037.0, 533082.0, 804744.0, 508880.0, 98385.0, 601905.0, 856620.0, 506559.0,
        220057.0, 562517.0, 844734.0, 507078.0, 150677.0, 580387.0, 558697.0, 522917.0, 621751.0,
        518719.0, 207067.0, 565321.0, 135297.0, 585677.0, 932968.0, 503404.0, 604456.0, 519822.0,
        579728.0, 521462.0, 244138.0, 557813.0, 706487.0, 513800.0, 711627.0, 513523.0, 853833.0,
        506674.0, 497220.0, 527562.0, 59428.0, 629511.0, 564845.0, 522486.0, 623621.0, 518603.0,
        242689.0, 558077.0, 125091.0, 589591.0, 363819.0, 540432.0, 686453.0, 514901.0, 656813.0,
        516594.0, 489901.0, 528155.0, 386380.0, 537905.0, 542819.0, 524052.0, 243987.0, 557841.0,
        693412.0, 514514.0, 488484.0, 528271.0, 896331.0, 504881.0, 336730.0, 543721.0, 728298.0,
        512647.0, 604215.0, 519840.0, 153729.0, 579413.0, 595687.0, 520398.0, 540360.0, 524240.0,
        245779.0, 557511.0, 924873.0, 503730.0, 509628.0, 526577.0, 528523.0, 525122.0, 3509.0,
        847707.0, 522756.0, 525555.0, 895447.0, 504922.0, 44840.0, 646067.0, 45860.0, 644715.0,
        463487.0, 530404.0, 398164.0, 536654.0, 894483.0, 504959.0, 619415.0, 518874.0, 966306.0,
        502129.0, 990922.0, 501212.0, 835756.0, 507474.0, 548881.0, 523618.0, 453578.0, 531282.0,
        474993.0, 529410.0, 80085.0, 612879.0, 737091.0, 512193.0, 50789.0, 638638.0, 979768.0,
        501620.0, 792018.0, 509483.0, 665001.0, 516122.0, 86552.0, 608694.0, 462772.0, 530469.0,
        589233.0, 520821.0, 891694.0, 505072.0, 592605.0, 520594.0, 209645.0, 564741.0, 42531.0,
        649269.0, 554376.0, 523226.0, 803814.0, 508929.0, 334157.0, 544042.0, 175836.0, 572970.0,
        868379.0, 506051.0, 658166.0, 516520.0, 278203.0, 551995.0, 966198.0, 502126.0, 627162.0,
        518387.0, 296774.0, 549165.0, 311803.0, 547027.0, 843797.0, 507118.0, 702304.0, 514032.0,
        563875.0, 522553.0, 33103.0, 664910.0, 191932.0, 568841.0, 543514.0, 524006.0, 506835.0,
        526794.0, 868368.0, 506052.0, 847025.0, 506971.0, 678623.0, 515342.0, 876139.0, 505726.0,
        571997.0, 521984.0, 598632.0, 520198.0, 213590.0, 563892.0, 625404.0, 518497.0, 726508.0,
        512738.0, 689426.0, 514738.0, 332495.0, 544264.0, 411366.0, 535302.0, 242546.0, 558110.0,
        315209.0, 546555.0, 797544.0, 509219.0, 93889.0, 604371.0, 858879.0, 506454.0, 124906.0,
        589666.0, 449072.0, 531693.0, 235960.0, 559345.0, 642403.0, 517454.0, 720567.0, 513047.0,
        705534.0, 513858.0, 603692.0, 519870.0, 488137.0, 528302.0, 157370.0, 578285.0, 63515.0,
        625730.0, 666326.0, 516041.0, 619226.0, 518883.0, 443613.0, 532186.0, 597717.0, 520257.0,
        96225.0, 603069.0, 86940.0, 608450.0, 40725.0, 651929.0, 460976.0, 530625.0, 268875.0,
        553508.0, 270671.0, 553214.0, 363254.0, 540500.0, 384248.0, 538137.0, 762889.0, 510892.0,
        377941.0, 538833.0, 278878.0, 551890.0, 176615.0, 572755.0, 860008.0, 506412.0, 944392.0,
        502967.0, 608395.0, 519571.0, 225283.0, 561450.0, 45095.0, 645728.0, 333798.0, 544090.0,
        625733.0, 518476.0, 995584.0, 501037.0, 506135.0, 526853.0, 238050.0, 558952.0, 557943.0,
        522972.0, 530978.0, 524938.0, 634244.0, 517949.0, 177168.0, 572616.0, 85200.0, 609541.0,
        953043.0, 502630.0, 523661.0, 525484.0, 999295.0, 500902.0, 840803.0, 507246.0, 961490.0,
        502312.0, 471747.0, 529685.0, 380705.0, 538523.0, 911180.0, 504275.0, 334149.0, 544046.0,
        478992.0, 529065.0, 325789.0, 545133.0, 335884.0, 543826.0, 426976.0, 533760.0, 749007.0,
        511582.0, 667067.0, 516000.0, 607586.0, 519623.0, 674054.0, 515599.0, 188534.0, 569675.0,
        565185.0, 522464.0, 172090.0, 573988.0, 87592.0, 608052.0, 907432.0, 504424.0, 8912.0,
        760841.0, 928318.0, 503590.0, 757917.0, 511138.0, 718693.0, 513153.0, 315141.0, 546566.0,
        728326.0, 512645.0, 353492.0, 541647.0, 638429.0, 517695.0, 628892.0, 518280.0, 877286.0,
        505672.0, 620895.0, 518778.0, 385878.0, 537959.0, 423311.0, 534113.0, 633501.0, 517997.0,
        884833.0, 505360.0, 883402.0, 505416.0, 999665.0, 500894.0, 708395.0, 513697.0, 548142.0,
        523667.0, 756491.0, 511205.0, 987352.0, 501340.0, 766520.0, 510705.0, 591775.0, 520647.0,
        833758.0, 507563.0, 843890.0, 507108.0, 925551.0, 503698.0, 74816.0, 616598.0, 646942.0,
        517187.0, 354923.0, 541481.0, 256291.0, 555638.0, 634470.0, 517942.0, 930904.0, 503494.0,
        134221.0, 586071.0, 282663.0, 551304.0, 986070.0, 501394.0, 123636.0, 590176.0, 123678.0,
        590164.0, 481717.0, 528841.0, 423076.0, 534137.0, 866246.0, 506145.0, 93313.0, 604697.0,
        783632.0, 509880.0, 317066.0, 546304.0, 502977.0, 527103.0, 141272.0, 583545.0, 71708.0,
        618938.0, 617748.0, 518975.0, 581190.0, 521362.0, 193824.0, 568382.0, 682368.0, 515131.0,
        352956.0, 541712.0, 351375.0, 541905.0, 505362.0, 526909.0, 905165.0, 504518.0, 128645.0,
        588188.0, 267143.0, 553787.0, 158409.0, 577965.0, 482776.0, 528754.0, 628896.0, 518282.0,
        485233.0, 528547.0, 563606.0, 522574.0, 111001.0, 595655.0, 115920.0, 593445.0, 365510.0,
        540237.0, 959724.0, 502374.0, 938763.0, 503184.0, 930044.0, 503520.0, 970959.0, 501956.0,
        913658.0, 504176.0, 68117.0, 621790.0, 989729.0, 501253.0, 567697.0, 522288.0, 820427.0,
        508163.0, 54236.0, 634794.0, 291557.0, 549938.0, 124961.0, 589646.0, 403177.0, 536130.0,
        405421.0, 535899.0, 410233.0, 535417.0, 815111.0, 508403.0, 213176.0, 563974.0, 83099.0,
        610879.0, 998588.0, 500934.0, 513640.0, 526263.0, 129817.0, 587733.0, 1820.0, 921851.0,
        287584.0, 550539.0, 299160.0, 548820.0, 860621.0, 506386.0, 529258.0, 525059.0, 586297.0,
        521017.0, 953406.0, 502616.0, 441234.0, 532410.0, 986217.0, 501386.0, 781938.0, 509957.0,
        461247.0, 530595.0, 735424.0, 512277.0, 146623.0, 581722.0, 839838.0, 507288.0, 510667.0,
        526494.0, 935085.0, 503327.0, 737523.0, 512167.0, 303455.0, 548204.0, 992779.0, 501145.0,
        60240.0, 628739.0, 939095.0, 503174.0, 794368.0, 509370.0, 501825.0, 527189.0, 459028.0,
        530798.0, 884641.0, 505363.0, 512287.0, 526364.0, 835165.0, 507499.0, 307723.0, 547590.0,
        160587.0, 577304.0, 735043.0, 512300.0, 493289.0, 527887.0, 110717.0, 595785.0, 306480.0,
        547772.0, 318593.0, 546089.0, 179810.0, 571911.0, 200531.0, 566799.0, 314999.0, 546580.0,
        197020.0, 567622.0, 301465.0, 548487.0, 237808.0, 559000.0, 131944.0, 586923.0, 882527.0,
        505449.0, 468117.0, 530003.0, 711319.0, 513541.0, 156240.0, 578628.0, 965452.0, 502162.0,
        992756.0, 501148.0, 437959.0, 532715.0, 739938.0, 512046.0, 614249.0, 519196.0, 391496.0,
        537356.0, 62746.0, 626418.0, 688215.0, 514806.0, 75501.0, 616091.0, 883573.0, 505412.0,
        558824.0, 522910.0, 759371.0, 511061.0, 173913.0, 573489.0, 891351.0, 505089.0, 727464.0,
        512693.0, 164833.0, 576051.0, 812317.0, 508529.0, 540320.0, 524243.0, 698061.0, 514257.0,
        69149.0, 620952.0, 471673.0, 529694.0, 159092.0, 577753.0, 428134.0, 533653.0, 89997.0,
        606608.0, 711061.0, 513557.0, 779403.0, 510081.0, 203327.0, 566155.0, 798176.0, 509187.0,
        667688.0, 515963.0, 636120.0, 517833.0, 137410.0, 584913.0, 217615.0, 563034.0, 556887.0,
        523038.0, 667229.0, 515991.0, 672276.0, 515708.0, 325361.0, 545187.0, 172115.0, 573985.0,
        13846.0, 725685.0,
    ];
    let (mut a, mut b, mut i, mut d, mut c) = (0.0, 0.0, 0, 0.0, 0.0);
    let mut bigpower = 0;
    let mut specialterm = 0.0;
    while (i != 2000) {
        a = nums[i];
        b = nums[i + 1];
        c = nums[bigpower];
        d = nums[bigpower + 1];
        let smallerpower = if d < b { d } else { b };
        b = b / smallerpower;
        d = d / smallerpower;
        if a.powf(b) > c.powf(d) {
            println!("{} {},{},{},{}", i, a, b, c, d);
            bigpower = i;
        }
        i += 2;
    }
    println!("{}", 1 + bigpower / 2);
    // GETTING OVERFLOW ERROR UNFIXABLE USE SOME OTHER METHOD
    // new approach Take smaller power and apply the reverse power on both sides
}
/*
OLD WAY
if b > d{
    specialterm = (a/c).powi(b as i32);*c.powf(b-d));
}
else{
    specialterm = (a/c).powi(d as i32);*(a.powf(b-d));
}
println!("a:{a},b:{b},c:{c}");
println!("{specialterm}");
if specialterm>1.0{
    bigpower = (i/2) + 1;
}
*/
