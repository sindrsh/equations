import inh;

real a = 60;
real h = 5;
real h2 = 40;
real h3 = 30;
real a2 = 20;
path p = box((0,0),(a,h));

pair D = (0,h);
pair C = (a,h);
pair Ep = (a/2,h+h2);
pair F = Ep+(0,h3);
pair G = (a+h2,F.y);

path p2 = D--Ep--C; 
path p3 = Ep--F--G;
draw(p);
draw(p2);
draw(p3);
draw(shift(G-(0,2))*scale(2)*unitcircle);
draw(reflect((G.x,-1),(G.x,1))*p);
draw(reflect((G.x,-1),(G.x,1))*p2);
draw(reflect((G.x,-1),(G.x,1))*p3);

write(G);

