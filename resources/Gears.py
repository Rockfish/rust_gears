#/usr/bin/python
import numpy as np
import matplotlib.path as mpath
import matplotlib.patches as mpatches
import matplotlib.pyplot as plt

from pylab import *

r = 10.0
step = 30.0
outside_circle = 12.5
pitch_circle = 10.0
root_circle = 10.0
pitch = 30.0



def rad(degree):
	return degree * pi / 180.0

def deg(rad):
	return rad * 180.0 / pi
	
def ycord(a, r):
	return sin(rad(a)) * r

def xcord(a, r):
	return cos(rad(a)) * r

def radius(angle, length):
	x = xcord(angle, length)
	y = ycord(angle, length)
	return [0.0, x], [0.0, y]

def tangent(angle, len, tlen=5.0):
	x = xcord(angle, len)
	y = ycord(angle, len)
	tan_angle = angle + 90.0
	tlen = step / 180.0 * pi * r
	tx = xcord(tan_angle, tlen)
	ty = ycord(tan_angle, tlen)
	tx += x
	ty += y
	plt.plot([x, tx], [y, ty], ':')

def involute(angle, len, direction=0):
	x = []
	y = []
	last = []
	
	if direction:
		start = angle
		end = angle + step + 5
		inc = 5
		right_angle = -90
	else:
		start = angle + step
		end = angle - 5
		inc = -5
		right_angle = 90
	
	#for s in range(start, end, inc):
	pitch_angle = start
	
	while 1:
		# base position - offsets
		bx = xcord(pitch_angle, len)
		by = ycord(pitch_angle, len)

		tan_angle = pitch_angle + right_angle
		tlen = abs(start - pitch_angle) / 180.0 * pi * r
		
		#print tlen
		
		tx = xcord(tan_angle, tlen)
		ty = ycord(tan_angle, tlen)
		
		tx += bx
		ty += by
		involute_radius = sqrt(tx*tx + ty*ty)
		
		if involute_radius > outside_circle:
			break
		
		#print involute_radius
		x.append(tx)
		y.append(ty)
		last = [tx, ty]
		#plt.plot([bx, tx], [by, ty])
		pitch_angle += inc
	
	plt.plot(x, y)
	return last

def gear(angle, len):
	s0, s1 = involute(angle, len)
	e0, e1 = involute(angle + step/2.0, len, 1)

	h = sqrt(s0**2 + s1**2)
	
	start_angle = deg(arcsin(s0/h))
	end_angle = deg(arcsin(e0/h))

	#if not (s0 > 0 and s1 < 0 and e0 > 0 and e1 < 0):return

	if s0 > 0 and s1 < 0 and e0 > 0 and e1 < 0:
		start_angle = 180 - start_angle
		end_angle = 180 - end_angle
	
	if s0 < 0 and s1 < 0 and e0 < 0 and e1 < 0:
		start_angle = 180 - start_angle
		end_angle = 180 - end_angle
	

	#print "start: %4.0f, %4.0f   end: %4.0f, %4.0f   angles: %6.2f, %6.2f" % (s0, s1, e0, e1, start_angle, end_angle)
	#plt.plot(s0, s1, 'ro')
	
	if start_angle > end_angle:
		cord_angles = linspace(end_angle, start_angle, num=20)
	else:
		cord_angles = linspace(start_angle, end_angle, num=20)
	
	#print cord_angles
	plt.plot(xcord(cord_angles, h), ycord(cord_angles, h))

def plot_radius(angle, length):
	x, y = radius(angle, length)
	plt.plot(x, y)
	

###

circle = 360

t1 = linspace(0.0, circle, num=360)

axis([-15.0, 15.0, -15.0, 15.0])
#axis([-65.0, 65.0, -65.0, 65.0])
#axis([0.0, 15.0, 0.0, 15.0])
#grid(True)

# Circle
plt.plot(xcord(t1, root_circle), ycord(t1, root_circle))
#plt.plot(xcord(t1,10.5), ycord(t1,10.5), '--')
#plt.plot(xcord(t1,11.0), ycord(t1,11.0), '--')
#plt.plot([0.0, 7.0], [0.0, 7.0], '-')

for i in range(0, 12):
	a = step * i
	print a
	plot_radius(a, root_circle)
	plot_radius(a+step/2.0, root_circle)
	#tangent(a, 10.0)
	#involute(a, 10.0)
	#involute(a + step/2.0, 10.0, 1)
	gear(a, pitch_circle)


plt.show()
