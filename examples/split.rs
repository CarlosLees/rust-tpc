fn main() {
    let string = "S168#865016401138730#0084#0017#SYNC:0078;STATUS:19,100$S168#865016401138730#0086#0194#LOCA:W;CELL:6,1cc,0,53b3,60f3892,30,540c,568bc0b,2d,540c,91f794c,2b,53b3,985bc16,28,540c,6ff694d,25,348c,9072a16,24;GDATA:V,0,2403070118
35,0.0,0.0,0,0,0;ALERT:0440;STATUS:19,99;WIFI:10,C6-B8-E6-D3-9E-4B,-69,C0-B8-E6-D3-9B-90,-72,58-41-20-0F-A4-AC,-72,62-93-8A-B7-8B-8F,-76,A4-1A-3A-48-83-7D,-77,C0-B8-E6-D3-9E-BA,-78,C6-B8-E6-D3-9E-BA,-78,FC-83-C6-06-63-26,-83,30-0D
-9E-FA-B3-3E,-83,C0-B8-E6-D3-9F-C2,-86$";

    let res:Vec<&str> = string.split('$').filter(|x| !x.is_empty()).collect();
    println!("{:?}", res);
}