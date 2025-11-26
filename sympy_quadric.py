# http://skuld.bmsc.washington.edu/people/merritt/graphics/quadrics.html
# Compute the big quadric equation with sympy

import sympy as smp

if __name__ == "__main__":
    A, B, C, D, E, F, G, H, I, J = smp.symbols("A B C D E F G H I J", real=True)
    x, y, z = smp.symbols("x y z", real=True)

    quadric = A*x*x + B*y*y + C*z*z + D*x*y + E*x*z + F*y*z + G*x + H*y + I*z + J

    O_x, O_y, O_z, D_x, D_y, D_z = smp.symbols(r"O_x O_y O_z D_x D_y D_z", real=True)

    t = smp.symbols("t", real=True)

    subs_quadric = quadric.subs(x, O_x + D_x*t).subs(y, O_y + D_y*t).subs(z, O_z + D_z*t).expand()

    collected_quadric = smp.collect(subs_quadric, t, evaluate=False)

    print("Quadratic equation by coeficients")
    print("t**2: ", collected_quadric[t**2])
    print("t   : ", collected_quadric[t]) # 2*A*D_x*O_x + 2*B*D_y*O_y + 2*C*D_z*O_z + D*D_x*O_y + D*D_y*O_x + D_x*E*O_z + D_x*G + D_y*F*O_z + D_y*H + D_z*E*O_x + D_z*F*O_y + D_z*I
    print("t**0: ", collected_quadric[1])

    print("Derivatives with respect to x, y, z")
    print(quadric.diff(x))
    print(quadric.diff(y))
    print(quadric.diff(z))