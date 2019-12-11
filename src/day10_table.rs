const ORIENTATION_TABLE: [[u16;61];61] = [
    [1947,1948,1963,1973,1983,1992,2002,2011,2020,2030,2039,2048,2058,2067,2078,2086,2094,2105,2114,2124,2133,2142,2152,2161,2170,2180,2189,2199,2209,2224,0001,0002,0017,0027,0037,0046,0056,0065,0074,0084,0093,0102,0112,0121,0132,0140,0148,0159,0168,0178,0187,0196,0206,0215,0224,0234,0243,0253,0263,0278,0279],
    [1946,1947,1949,1964,1974,1985,1993,2003,2015,2023,2033,2040,2052,2062,2072,2085,2087,2100,2110,2120,2132,2139,2149,2157,2169,2179,2187,2198,2208,2223,0001,0003,0018,0028,0039,0047,0057,0069,0077,0087,0094,0106,0116,0126,0139,0141,0154,0164,0174,0186,0193,0203,0211,0223,0233,0241,0252,0262,0277,0279,0280],
    [1931,1945,1947,1950,1965,1976,1986,1996,2006,2016,2026,2038,2045,2057,2066,2077,2086,2095,2106,2115,2127,2134,2146,2156,2166,2176,2186,2196,2207,2222,0001,0004,0019,0030,0040,0050,0060,0070,0080,0092,0099,0111,0120,0131,0140,0149,0160,0169,0181,0188,0200,0210,0220,0230,0240,0250,0261,0276,0279,0281,0295],
    [1921,1930,1944,1947,1951,1966,1977,1987,1998,2008,2017,2029,2039,2050,2059,2071,2084,2088,2101,2113,2122,2133,2143,2155,2164,2174,2185,2195,2206,2221,0001,0005,0020,0031,0041,0052,0062,0071,0083,0093,0104,0113,0125,0138,0142,0155,0167,0176,0187,0197,0209,0218,0228,0239,0249,0260,0275,0279,0282,0296,0305],
    [1911,1920,1929,1943,1947,1952,1967,1978,1989,2001,2010,2021,2032,2041,2054,2065,2076,2086,2096,2107,2118,2131,2140,2151,2162,2171,2183,2194,2205,2220,0001,0006,0021,0032,0043,0055,0064,0075,0086,0095,0108,0119,0130,0140,0150,0161,0172,0185,0194,0205,0216,0225,0237,0248,0259,0274,0279,0283,0297,0306,0315],
    [1902,1909,1918,1928,1942,1947,1953,1968,1980,1991,2002,2014,2025,2037,2046,2058,2070,2083,2089,2102,2114,2126,2135,2147,2158,2170,2181,2192,2204,2219,0001,0007,0022,0034,0045,0056,0068,0079,0091,0100,0112,0124,0137,0143,0156,0168,0180,0189,0201,0212,0224,0235,0246,0258,0273,0279,0284,0298,0308,0317,0324],
    [1892,1901,1908,1917,1927,1941,1947,1954,1969,1981,1992,2004,2016,2027,2039,2051,2063,2075,2086,2097,2109,2121,2133,2145,2156,2168,2180,2191,2203,2218,0001,0008,0023,0035,0046,0058,0070,0081,0093,0105,0117,0129,0140,0151,0163,0175,0187,0199,0210,0222,0234,0245,0257,0272,0279,0285,0299,0309,0318,0325,0334],
    [1883,1891,1898,1907,1916,1926,1940,1947,1955,1970,1982,1994,2007,2018,2031,2042,2056,2068,2082,2090,2104,2116,2130,2141,2154,2165,2178,2190,2202,2217,0001,0009,0024,0036,0048,0061,0072,0085,0096,0110,0122,0136,0144,0158,0170,0184,0195,0208,0219,0232,0244,0256,0271,0279,0286,0300,0310,0319,0328,0335,0343],
    [1874,1879,1888,1896,1905,1914,1925,1939,1947,1956,1971,1984,1997,2009,2022,2036,2047,2060,2074,2086,2098,2112,2125,2136,2150,2163,2175,2188,2201,2216,0001,0010,0025,0038,0051,0063,0076,0090,0101,0114,0128,0140,0152,0166,0179,0190,0204,0217,0229,0242,0255,0270,0279,0287,0301,0312,0321,0330,0338,0347,0352],
    [1864,1871,1878,1886,1893,1903,1913,1924,1938,1947,1957,1972,1986,2000,2013,2026,2039,2053,2066,2081,2091,2106,2119,2133,2146,2159,2172,2186,2200,2215,0001,0011,0026,0040,0054,0067,0080,0093,0107,0120,0135,0145,0160,0173,0187,0200,0213,0226,0240,0254,0269,0279,0288,0302,0313,0323,0333,0340,0348,0355,0362],
    [1855,1861,1868,1877,1884,1892,1902,1912,1923,1937,1947,1958,1973,1988,2002,2016,2030,2043,2058,2073,2086,2099,2114,2129,2142,2156,2170,2184,2199,2214,0001,0012,0027,0042,0056,0070,0084,0097,0112,0127,0140,0153,0168,0183,0196,0210,0224,0238,0253,0268,0279,0289,0303,0314,0324,0334,0342,0349,0358,0365,0371],
    [1846,1854,1856,1865,1873,1880,1890,1900,1910,1922,1936,1947,1959,1975,1990,2005,2019,2035,2049,2064,2080,2092,2108,2123,2137,2153,2167,2182,2197,2213,0001,0013,0029,0044,0059,0073,0089,0103,0118,0134,0146,0162,0177,0191,0207,0221,0236,0251,0267,0279,0290,0304,0316,0326,0336,0346,0353,0361,0370,0372,0380],
    [1836,1842,1849,1855,1862,1869,1878,1887,1897,1908,1921,1935,1947,1960,1977,1992,2008,2024,2039,2055,2071,2086,2101,2117,2133,2148,2164,2180,2195,2212,0001,0014,0031,0046,0062,0078,0093,0109,0125,0140,0155,0171,0187,0202,0218,0234,0249,0266,0279,0291,0305,0318,0329,0339,0348,0357,0364,0371,0377,0384,0390],
    [1827,1832,1837,1844,1853,1857,1867,1876,1885,1894,1906,1919,1934,1947,1961,1979,1995,2012,2028,2044,2061,2079,2093,2111,2128,2144,2160,2177,2193,2211,0001,0015,0033,0049,0066,0082,0098,0115,0133,0147,0165,0182,0198,0214,0231,0247,0265,0279,0292,0307,0320,0332,0341,0350,0359,0369,0373,0382,0389,0394,0399],
    [1816,1822,1828,1835,1840,1848,1855,1863,1872,1881,1892,1904,1917,1933,1947,1962,1981,1999,2016,2034,2051,2069,2086,2103,2121,2138,2156,2173,2191,2210,0001,0016,0035,0053,0070,0088,0105,0123,0140,0157,0175,0192,0210,0227,0245,0264,0279,0293,0309,0322,0334,0345,0354,0363,0371,0378,0386,0391,0398,0404,0410],
    [1808,1809,1817,1823,1829,1836,1843,1852,1858,1868,1878,1889,1902,1915,1932,1947,1963,1983,2002,2020,2039,2058,2078,2094,2114,2133,2152,2170,2189,2209,0001,0017,0037,0056,0074,0093,0112,0132,0148,0168,0187,0206,0224,0243,0263,0279,0294,0311,0324,0337,0348,0358,0368,0374,0383,0390,0397,0403,0409,0417,0418],
    [1800,1807,1808,1810,1818,1824,1831,1838,1847,1855,1864,1875,1886,1899,1913,1931,1947,1965,1986,2006,2026,2045,2066,2086,2106,2127,2146,2166,2186,2207,0001,0019,0040,0060,0080,0099,0120,0140,0160,0181,0200,0220,0240,0261,0279,0295,0313,0327,0340,0351,0362,0371,0379,0388,0395,0402,0408,0416,0418,0419,0426],
    [1789,1794,1799,1806,1808,1811,1819,1826,1834,1841,1851,1859,1870,1882,1895,1911,1929,1947,1967,1989,2010,2032,2054,2076,2096,2118,2140,2162,2183,2205,0001,0021,0043,0064,0086,0108,0130,0150,0172,0194,0216,0237,0259,0279,0297,0315,0331,0344,0356,0367,0375,0385,0392,0400,0407,0415,0418,0420,0427,0432,0437],
    [1780,1784,1788,1793,1798,1805,1808,1812,1820,1828,1836,1845,1855,1866,1878,1892,1908,1927,1947,1969,1992,2016,2039,2063,2086,2109,2133,2156,2180,2203,0001,0023,0046,0070,0093,0117,0140,0163,0187,0210,0234,0257,0279,0299,0318,0334,0348,0360,0371,0381,0390,0398,0406,0414,0418,0421,0428,0433,0438,0442,0446],
    [1770,1774,1779,1781,1787,1792,1797,1804,1808,1813,1821,1830,1839,1850,1860,1874,1888,1905,1925,1947,1971,1997,2022,2047,2074,2098,2125,2150,2175,2201,0001,0025,0051,0076,0101,0128,0152,0179,0204,0229,0255,0279,0301,0321,0338,0352,0366,0376,0387,0396,0405,0413,0418,0422,0429,0434,0439,0445,0447,0452,0456],
    [1761,1762,1767,1772,1776,1780,1785,1790,1796,1803,1808,1814,1823,1833,1843,1855,1868,1884,1902,1923,1947,1973,2002,2030,2058,2086,2114,2142,2170,2199,0001,0027,0056,0084,0112,0140,0168,0196,0224,0253,0279,0303,0324,0342,0358,0371,0383,0393,0403,0412,0418,0423,0430,0436,0441,0446,0450,0454,0459,0464,0465],
    [1752,1755,1760,1761,1763,1768,1773,1778,1782,1788,1795,1802,1808,1815,1825,1836,1849,1862,1878,1897,1921,1947,1977,2008,2039,2071,2101,2133,2164,2195,0001,0031,0062,0093,0125,0155,0187,0218,0249,0279,0305,0329,0348,0364,0377,0390,0401,0411,0418,0424,0431,0438,0444,0448,0453,0458,0463,0465,0466,0471,0474],
    [1742,1745,1748,1751,1754,1759,1761,1764,1769,1775,1780,1786,1793,1801,1808,1816,1828,1840,1855,1872,1892,1917,1947,1981,2016,2051,2086,2121,2156,2191,0001,0035,0070,0105,0140,0175,0210,0245,0279,0309,0334,0354,0371,0386,0398,0410,0418,0425,0433,0440,0446,0451,0457,0462,0465,0467,0472,0475,0478,0481,0484],
    [1733,1737,1738,1739,1743,1747,1749,1753,1758,1761,1765,1771,1777,1783,1791,1800,1808,1818,1831,1847,1864,1886,1913,1947,1986,2026,2066,2106,2146,2186,0001,0040,0080,0120,0160,0200,0240,0279,0313,0340,0362,0379,0395,0408,0418,0426,0435,0443,0449,0455,0461,0465,0468,0473,0477,0479,0483,0487,0488,0489,0493],
    [1724,1725,1728,1730,1732,1736,1738,1740,1744,1748,1752,1757,1761,1766,1773,1780,1788,1798,1808,1820,1836,1855,1878,1908,1947,1992,2039,2086,2133,2180,0001,0046,0093,0140,0187,0234,0279,0318,0348,0371,0390,0406,0418,0428,0438,0446,0453,0460,0465,0469,0474,0478,0482,0486,0488,0490,0494,0496,0498,0501,0502],
    [1714,1715,1718,1720,1723,1724,1726,1729,1731,1735,1738,1741,1746,1750,1756,1761,1767,1776,1785,1796,1808,1823,1843,1868,1902,1947,2002,2058,2114,2170,0001,0056,0112,0168,0224,0279,0324,0358,0383,0403,0418,0430,0441,0450,0459,0465,0470,0476,0480,0485,0488,0491,0495,0497,0500,0502,0503,0506,0508,0511,0512],
    [1705,1707,1708,1709,1711,1713,1714,1716,1719,1722,1724,1727,1730,1734,1738,1742,1748,1754,1761,1769,1780,1793,1808,1828,1855,1892,1947,2016,2086,2156,0001,0070,0140,0210,0279,0334,0371,0398,0418,0433,0446,0457,0465,0472,0478,0484,0488,0492,0496,0499,0502,0504,0507,0510,0512,0513,0515,0517,0518,0519,0521],
    [1695,1696,1698,1699,1700,1702,1703,1704,1706,1708,1710,1712,1714,1717,1721,1724,1728,1732,1738,1744,1752,1761,1773,1788,1808,1836,1878,1947,2039,2133,0001,0093,0187,0279,0348,0390,0418,0438,0453,0465,0474,0482,0488,0494,0498,0502,0505,0509,0512,0514,0516,0518,0520,0522,0523,0524,0526,0527,0528,0530,0531],
    [1685,1686,1687,1688,1689,1690,1691,1692,1693,1694,1695,1697,1699,1701,1703,1705,1708,1711,1714,1719,1724,1730,1738,1748,1761,1780,1808,1855,1947,2086,0001,0140,0279,0371,0418,0446,0465,0478,0488,0496,0502,0507,0512,0515,0518,0521,0523,0525,0527,0529,0531,0532,0533,0534,0535,0536,0537,0538,0539,0540,0541],
    [1670,1671,1672,1673,1674,1675,1676,1677,1678,1679,1680,1681,1682,1683,1684,1685,1687,1689,1691,1693,1695,1699,1703,1708,1714,1724,1738,1761,1808,1947,0001,0279,0418,0465,0488,0502,0512,0518,0523,0527,0531,0533,0535,0537,0539,0541,0542,0543,0544,0545,0546,0547,0548,0549,0550,0551,0552,0553,0554,0555,0556],
    [1669,1669,1669,1669,1669,1669,1669,1669,1669,1669,1669,1669,1669,1669,1669,1669,1669,1669,1669,1669,1669,1669,1669,1669,1669,1669,1669,1669,1669,1669,0000,0557,0557,0557,0557,0557,0557,0557,0557,0557,0557,0557,0557,0557,0557,0557,0557,0557,0557,0557,0557,0557,0557,0557,0557,0557,0557,0557,0557,0557,0557],
    [1668,1667,1666,1665,1664,1663,1662,1661,1660,1659,1658,1657,1656,1655,1654,1653,1651,1649,1647,1645,1643,1639,1635,1630,1624,1614,1600,1577,1530,1391,1113,0835,0696,0649,0626,0612,0602,0596,0591,0587,0583,0581,0579,0577,0575,0573,0572,0571,0570,0569,0568,0567,0566,0565,0564,0563,0562,0561,0560,0559,0558],
    [1653,1652,1651,1650,1649,1648,1647,1646,1645,1644,1643,1641,1639,1637,1635,1633,1630,1627,1624,1619,1614,1608,1600,1590,1577,1558,1530,1483,1391,1252,1113,0974,0835,0743,0696,0668,0649,0636,0626,0618,0612,0607,0602,0599,0596,0593,0591,0589,0587,0585,0583,0582,0581,0580,0579,0578,0577,0576,0575,0574,0573],
    [1643,1642,1640,1639,1638,1636,1635,1634,1632,1630,1628,1626,1624,1621,1617,1614,1610,1606,1600,1594,1586,1577,1565,1550,1530,1502,1460,1391,1299,1205,1113,1021,0927,0835,0766,0724,0696,0676,0661,0649,0640,0632,0626,0620,0616,0612,0609,0605,0602,0600,0598,0596,0594,0592,0591,0590,0588,0587,0586,0584,0583],
    [1633,1631,1630,1629,1627,1625,1624,1622,1619,1616,1614,1611,1608,1604,1600,1596,1590,1584,1577,1569,1558,1545,1530,1510,1483,1446,1391,1322,1252,1182,1113,1044,0974,0904,0835,0780,0743,0716,0696,0681,0668,0657,0649,0642,0636,0630,0626,0622,0618,0615,0612,0610,0607,0604,0602,0601,0599,0597,0596,0595,0593],
    [1624,1623,1620,1618,1615,1614,1612,1609,1607,1603,1600,1597,1592,1588,1582,1577,1571,1562,1553,1542,1530,1515,1495,1470,1436,1391,1336,1280,1224,1168,1113,1058,1002,0946,0890,0835,0790,0756,0731,0711,0696,0684,0673,0664,0655,0649,0644,0638,0634,0629,0626,0623,0619,0617,0614,0612,0611,0608,0606,0603,0602],
    [1614,1613,1610,1608,1606,1602,1600,1598,1594,1590,1586,1581,1577,1572,1565,1558,1550,1540,1530,1518,1502,1483,1460,1430,1391,1346,1299,1252,1205,1158,1113,1068,1021,0974,0927,0880,0835,0796,0766,0743,0724,0708,0696,0686,0676,0668,0661,0654,0649,0645,0640,0636,0632,0628,0626,0624,0620,0618,0616,0613,0612],
    [1605,1601,1600,1599,1595,1591,1589,1585,1580,1577,1573,1567,1561,1555,1547,1538,1530,1520,1507,1491,1474,1452,1425,1391,1352,1312,1272,1232,1192,1152,1113,1074,1034,0994,0954,0914,0874,0835,0801,0774,0752,0735,0719,0706,0696,0688,0679,0671,0665,0659,0653,0649,0646,0641,0637,0635,0631,0627,0626,0625,0621],
    [1596,1593,1590,1587,1584,1579,1577,1574,1569,1563,1558,1552,1545,1537,1530,1522,1510,1498,1483,1466,1446,1421,1391,1357,1322,1287,1252,1217,1182,1147,1113,1079,1044,1009,0974,0939,0904,0869,0835,0805,0780,0760,0743,0728,0716,0704,0696,0689,0681,0674,0668,0663,0657,0652,0649,0647,0642,0639,0636,0633,0630],
    [1586,1583,1578,1577,1575,1570,1565,1560,1556,1550,1543,1536,1530,1523,1513,1502,1489,1476,1460,1441,1417,1391,1361,1330,1299,1267,1237,1205,1174,1143,1113,1083,1052,1021,0989,0959,0927,0896,0865,0835,0809,0785,0766,0750,0737,0724,0713,0703,0696,0690,0683,0676,0670,0666,0661,0656,0651,0649,0648,0643,0640],
    [1577,1576,1571,1566,1562,1558,1553,1548,1542,1535,1530,1524,1515,1505,1495,1483,1470,1454,1436,1415,1391,1365,1336,1308,1280,1252,1224,1196,1168,1139,1113,1087,1058,1030,1002,0974,0946,0918,0890,0861,0835,0811,0790,0772,0756,0743,0731,0721,0711,0702,0696,0691,0684,0678,0673,0668,0664,0660,0655,0650,0649],
    [1568,1564,1559,1557,1551,1546,1541,1534,1530,1525,1517,1508,1499,1488,1478,1464,1450,1433,1413,1391,1367,1341,1316,1291,1264,1240,1213,1188,1163,1137,1113,1089,1063,1038,1013,0986,0962,0935,0910,0885,0859,0835,0813,0793,0776,0762,0748,0738,0727,0718,0709,0701,0696,0692,0685,0680,0675,0669,0667,0662,0658],
    [1558,1554,1550,1545,1540,1533,1530,1526,1518,1510,1502,1493,1483,1472,1460,1446,1430,1411,1391,1369,1346,1322,1299,1275,1252,1229,1205,1182,1158,1135,1113,1091,1068,1044,1021,0997,0974,0951,0927,0904,0880,0857,0835,0815,0796,0780,0766,0754,0743,0733,0724,0716,0708,0700,0696,0693,0686,0681,0676,0672,0668],
    [1549,1544,1539,1532,1530,1527,1519,1512,1504,1497,1487,1479,1468,1456,1443,1427,1409,1391,1371,1349,1328,1306,1284,1262,1242,1220,1198,1176,1155,1133,1113,1093,1071,1050,1028,1006,0984,0964,0942,0920,0898,0877,0855,0835,0817,0799,0783,0770,0758,0747,0739,0729,0722,0714,0707,0699,0696,0694,0687,0682,0677],
    [1538,1531,1530,1528,1520,1514,1507,1500,1491,1483,1474,1463,1452,1439,1425,1407,1391,1373,1352,1332,1312,1293,1272,1252,1232,1211,1192,1172,1152,1131,1113,1095,1074,1054,1034,1015,0994,0974,0954,0933,0914,0894,0874,0853,0835,0819,0801,0787,0774,0763,0752,0743,0735,0726,0719,0712,0706,0698,0696,0695,0688],
    [1530,1529,1521,1515,1509,1502,1495,1486,1480,1470,1460,1449,1436,1423,1406,1391,1375,1355,1336,1318,1299,1280,1260,1244,1224,1205,1186,1168,1149,1129,1113,1097,1077,1058,1040,1021,1002,0982,0966,0946,0927,0908,0890,0871,0851,0835,0820,0803,0790,0777,0766,0756,0746,0740,0731,0724,0717,0711,0705,0697,0696],
    [1522,1516,1510,1503,1498,1490,1483,1475,1466,1457,1446,1434,1421,1405,1391,1376,1357,1339,1322,1304,1287,1269,1252,1235,1217,1200,1182,1165,1147,1128,1113,1098,1079,1061,1044,1026,1009,0991,0974,0957,0939,0922,0904,0887,0869,0850,0835,0821,0805,0792,0780,0769,0760,0751,0743,0736,0728,0723,0716,0710,0704],
    [1511,1506,1501,1494,1485,1481,1471,1462,1453,1444,1432,1419,1404,1391,1377,1359,1343,1326,1310,1294,1277,1259,1245,1227,1210,1194,1178,1161,1145,1127,1113,1099,1081,1065,1048,1032,1016,0999,0981,0967,0949,0932,0916,0900,0883,0867,0849,0835,0822,0807,0794,0782,0773,0764,0755,0745,0741,0732,0725,0720,0715],
    [1502,1496,1489,1483,1476,1469,1460,1451,1441,1430,1417,1403,1391,1378,1361,1346,1330,1314,1299,1283,1267,1252,1237,1221,1205,1190,1174,1158,1143,1126,1113,1100,1083,1068,1052,1036,1021,1005,0989,0974,0959,0943,0927,0912,0896,0880,0865,0848,0835,0823,0809,0796,0785,0775,0766,0757,0750,0743,0737,0730,0724],
    [1492,1484,1482,1473,1465,1458,1448,1438,1428,1416,1402,1391,1379,1363,1348,1333,1319,1303,1289,1274,1258,1246,1230,1215,1201,1185,1171,1156,1141,1125,1113,1101,1085,1070,1055,1041,1025,1011,0996,0980,0968,0952,0937,0923,0907,0893,0878,0863,0847,0835,0824,0810,0798,0788,0778,0768,0761,0753,0744,0742,0734],
    [1483,1477,1470,1461,1454,1446,1436,1426,1415,1401,1391,1380,1365,1350,1336,1322,1308,1295,1280,1265,1252,1239,1224,1209,1196,1182,1168,1154,1139,1124,1113,1102,1087,1072,1058,1044,1030,1017,1002,0987,0974,0961,0946,0931,0918,0904,0890,0876,0861,0846,0835,0825,0811,0800,0790,0780,0772,0765,0756,0749,0743],
    [1474,1467,1460,1452,1445,1435,1425,1414,1400,1391,1381,1366,1352,1338,1325,1312,1299,1285,1272,1257,1247,1232,1219,1205,1192,1179,1166,1152,1138,1123,1113,1103,1088,1074,1060,1047,1034,1021,1007,0994,0979,0969,0954,0941,0927,0914,0901,0888,0874,0860,0845,0835,0826,0812,0801,0791,0781,0774,0766,0759,0752],
    [1464,1459,1450,1442,1433,1424,1413,1399,1391,1382,1367,1354,1341,1329,1316,1302,1291,1278,1264,1252,1240,1226,1213,1202,1188,1175,1163,1150,1137,1122,1113,1104,1089,1076,1063,1051,1038,1024,1013,1000,0986,0974,0962,0948,0935,0924,0910,0897,0885,0872,0859,0844,0835,0827,0813,0802,0793,0784,0776,0767,0762],
    [1455,1447,1440,1431,1422,1412,1398,1391,1383,1368,1356,1344,1331,1320,1307,1296,1282,1270,1256,1248,1234,1222,1208,1197,1184,1173,1160,1148,1136,1121,1113,1105,1090,1078,1066,1053,1042,1029,1018,1004,0992,0978,0970,0956,0944,0930,0919,0906,0895,0882,0870,0858,0843,0835,0828,0814,0804,0795,0786,0779,0771],
    [1446,1437,1430,1421,1411,1397,1391,1384,1369,1357,1346,1334,1322,1311,1299,1287,1275,1263,1252,1241,1229,1217,1205,1193,1182,1170,1158,1147,1135,1120,1113,1106,1091,1079,1068,1056,1044,1033,1021,1009,0997,0985,0974,0963,0951,0939,0927,0915,0904,0892,0880,0869,0857,0842,0835,0829,0815,0805,0796,0789,0780],
    [1436,1429,1420,1410,1396,1391,1385,1370,1358,1347,1336,1324,1313,1301,1292,1280,1268,1255,1249,1236,1224,1212,1203,1191,1180,1168,1157,1146,1134,1119,1113,1107,1092,1080,1069,1058,1046,1035,1023,1014,1002,0990,0977,0971,0958,0946,0934,0925,0913,0902,0890,0879,0868,0856,0841,0835,0830,0816,0806,0797,0790],
    [1427,1418,1409,1395,1391,1386,1371,1360,1349,1337,1328,1317,1306,1297,1284,1273,1262,1252,1242,1231,1220,1207,1198,1187,1176,1167,1155,1144,1133,1118,1113,1108,1093,1082,1071,1059,1050,1039,1028,1019,1006,0995,0984,0974,0964,0953,0942,0929,0920,0909,0898,0889,0877,0866,0855,0840,0835,0831,0817,0808,0799],
    [1417,1408,1394,1391,1387,1372,1361,1351,1340,1330,1321,1309,1299,1288,1279,1267,1254,1250,1237,1225,1216,1205,1195,1183,1174,1164,1153,1143,1132,1117,1113,1109,1094,1083,1073,1062,1052,1043,1031,1021,1010,1001,0989,0976,0972,0959,0947,0938,0927,0917,0905,0896,0886,0875,0865,0854,0839,0835,0832,0818,0809],
    [1407,1393,1391,1388,1373,1362,1352,1342,1332,1322,1312,1300,1293,1281,1272,1261,1252,1243,1232,1223,1211,1204,1192,1182,1172,1162,1152,1142,1131,1116,1113,1110,1095,1084,1074,1064,1054,1044,1034,1022,1015,1003,0994,0983,0974,0965,0954,0945,0933,0926,0914,0904,0894,0884,0874,0864,0853,0838,0835,0833,0819],
    [1392,1391,1389,1374,1364,1353,1345,1335,1323,1315,1305,1298,1286,1276,1266,1253,1251,1238,1228,1218,1206,1199,1189,1181,1169,1159,1151,1140,1130,1115,1113,1111,1096,1086,1075,1067,1057,1045,1037,1027,1020,1008,0998,0988,0975,0973,0960,0950,0940,0928,0921,0911,0903,0891,0881,0873,0862,0852,0837,0835,0834],
    [1391,1390,1375,1365,1355,1346,1336,1327,1318,1308,1299,1290,1280,1271,1260,1252,1244,1233,1224,1214,1205,1196,1186,1177,1168,1158,1149,1139,1129,1114,1113,1112,1097,1087,1077,1068,1058,1049,1040,1030,1021,1012,1002,0993,0982,0974,0966,0955,0946,0936,0927,0918,0908,0899,0890,0880,0871,0861,0851,0836,0835],
];