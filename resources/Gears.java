/*
 * 	Involute Gears
 * 
 * 	Simple for drawing involute gears.
 * 
 * 	John Hatch - Rockfish LLC.
 * 
 */

import java.awt.Color;
import java.awt.Graphics;
import java.awt.Graphics2D;
import java.awt.RenderingHints;
import java.awt.geom.*;
import static java.lang.Math.*;



@SuppressWarnings("serial")
public class Gears extends javax.swing.JComponent {

	double radius = 200.0;
	int step = 30;
	int outside_circle = 230;
	double pitch_circle = 200.0;
	double base_circle = 0;
	double root_circle = 175;
	double involute_offset = 0;
	
	double pressure_angle = 20.0;
		
	double pitch = 30.0;
	int xoffset = 250;
	int yoffset = 250;
	
	int spin = 0;
	boolean newPath = true;
	
	// 
	// To draw the involute in the right position we need to know where to start so that it intersects
	// the pitch circle in the right place. 
	// 
	// Calculate length of tangent from root_circle to pitch_circle:
	//
	// 		Start with right triangle who's base, or adjacent, length equals root_circle radius
	//      and a hypotenuse that equals pitch_circle radius.  
	//      The acute angle of the triangle is arccos(root_circle/pitch_cirle).
	//
	//      From the angle, the opposite side, which is the tangent equals sin(angle)*pitch_circle.
	//
	// The tangent length is proportional to the angle on the root_circle for the value of the involute at that point.
	//
	//		The angle for the involute off the root_circle is the root_angle which equals tangent_length/root_circle in radians.
	//		That angle is length of the arc on the root circle between the involute's origin and where it intersects the pitch circle.
	//
	// The difference between the root_angle and the original angle is the offset we need to know to start the involute
	// so that it will intersect the pitch circle at the right point.
	//
	// 
	
	public double set_involute_offset() {
		double tan_intersect_angle = 0;
		double tangent_length = 0;
		double root_angle = 0;
		
		// Set the root_circle from the pitch_circle and pressure_angle
		base_circle = pitch_circle * cos(rad(pressure_angle));
		
		System.out.println("base_circle: " + base_circle );

		// Length of tangent from root circle to pitch circle.
		tan_intersect_angle = acos(root_circle/pitch_circle);
		tangent_length = sin(tan_intersect_angle) * pitch_circle;
		
		// Angle = arc / radius
		// Angle in radians where the involute intersects the pitch circle
		root_angle = tangent_length / root_circle;
		
		//tan_intersect_angle = deg(tan_intersect_angle);
		//root_angle = deg(root_angle);
		
		// Offset is the distance between the angle on the pitch circle and 
		// where the involute needs to start at to intersect pitch circle correctly. 
		involute_offset = deg(root_angle - tan_intersect_angle);
		
		// Outside circle intersection
		tan_intersect_angle = acos(root_circle/outside_circle);
		tangent_length = sin(tan_intersect_angle) * outside_circle;
		
		// Angle in radians where the involute intersects the outside circle
		// Can use this angle to calculate the endpoints of the involute curves.
		root_angle = tangent_length / root_circle;

		return root_angle;		
	}
	
	public GeneralPath gear() {
		newPath = true;
		GeneralPath p = new GeneralPath(GeneralPath.WIND_EVEN_ODD);
		for (int i = 12; i>0; i--) {
			int angle = i * step; // + spin;
			tooth(p, angle, root_circle);
		}
		p.closePath();
		return p;
	}

	public GeneralPath involute(double angle, double len, boolean direction) {
		
		GeneralPath p = new GeneralPath(GeneralPath.WIND_EVEN_ODD);

		double start, inc, right_angle;
		
		if (direction) {
			start = angle;
			inc = 5;
			right_angle = -90; 
		} else {
			start = angle + step;
			inc = -5;
			right_angle = 90; 
		}
		
		double pitch_angle = start;
		boolean first = true;
		while (true) {
			//# base position - offsets
			double bx = xcord(pitch_angle, len);
			double by = ycord(pitch_angle, len);

			double tan_angle = pitch_angle + right_angle;
			double tan_len = abs(start - pitch_angle) / 180.0 * PI * root_circle;
			pitch_angle += inc;
			
			double tx = xcord(tan_angle, tan_len);
			double ty = ycord(tan_angle, tan_len);
			
			tx += bx;
			ty += by;
			double involute_radius = sqrt(tx*tx + ty*ty);
			
			if (involute_radius > outside_circle)
				break;
			
			if (first) {
				p.moveTo(tx, ty);
				first = false;
			} else {
				p.lineTo(tx, ty);
			}
		}
		return p;
	}

	public void tooth(GeneralPath p, double angle, double len) {
		
		// Draw frontside and backside involute surfaces 
		GeneralPath i1 = involute(angle + involute_offset, len, false);
		GeneralPath i2 = involute(angle + step/2.0 - involute_offset, len, true);
		
		//Point2D s = i1.getCurrentPoint();
		//Point2D e = i2.getCurrentPoint();
		
		PathIterator pi1 = i1.getPathIterator(null);
		ReversePathIterator rpi2 = new ReversePathIterator( i2.getPathIterator(null) );

		// Insert the dedendum here by appending:
		//
		// 		coord(angle, base_circle)
		//		coord(angle, root_circle)
		//		coord(angle+step/2.0, root_circle)
		//		coord(angle+step/2.0, base_circle)
		//
		
		// Between these appends is where we would add an arc path for curve teeth tops
		p.append(pi1, true);
		p.append(rpi2, true);

	}
	
// Calculations for arc path of teeth tops
// Not used. Instead the appends connect them with straight lines.
//
//		double involute_len = sqrt(Math.pow(s.getX(), 2) + Math.pow(s.getY(),2));
//
//		double start_angle = deg(asin(s.getX()/involute_len));
//		double end_angle = deg(asin(e.getX()/involute_len));
//
//		if ( s.getX() > 0 && s.getY() < 0 && e.getX() > 0 && e.getY() < 0) {
//			start_angle = 180 - start_angle;
//			end_angle = 180 - end_angle;
//		}
//		
//		if (s.getX() < 0 && s.getY() < 0 && e.getX() < 0 && e.getY() < 0) {
//			start_angle = 180 - start_angle;
//			end_angle = 180 - end_angle;
//		}
//
//		// Connect teeth at top
//		if (start_angle > end_angle) {
//			//plot_gear(p, end_angle, start_angle, 20, involute_len);
//		}
//		else {
//			//plot_gear(p, start_angle, end_angle, 20, involute_len);
//		}
//	}

//	public void plot_gear(GeneralPath p, double start, double end, double step, double length) {
//		for (double angle = start; angle < end; angle += step) {
//			double x = xcord(angle, length);
//			double y = ycord(angle, length);
//			p.lineTo(x, y);
//		}
//	}
	
	public void paint(Graphics gd) {
		java.awt.Graphics2D g = (Graphics2D) gd;
		g.setRenderingHint(RenderingHints.KEY_ANTIALIASING, RenderingHints.VALUE_ANTIALIAS_ON);

		AffineTransform saveAT = g.getTransform();

		// Calculate the values we need and draw the gear as a path.
		set_involute_offset();
		GeneralPath gearPath = gear();

		// Clear background
		g.setColor(Color.WHITE);
		g.fillRect(0, 0, 1200, 550);

		// Position first gear 
		g.translate(250, 250);
		g.rotate(rad(spin));
		
		g.setColor(Color.red);
		circle(g, pitch_circle);
		g.setColor(Color.black);
		g.draw(gearPath);

		// Restore original transform
		g.setTransform(saveAT);

		// Position second gear
		xoffset = (int) round(pitch_circle * 2);
		
		g.translate(250+xoffset, 250);

		// Adjust phase of second gear
		g.rotate(rad(360/12));
		// Spin second gear
		g.rotate(rad(-spin));

		g.setColor(Color.red);
		circle(g, pitch_circle);
		g.setColor(Color.blue);
		g.draw(gearPath);

		// Rotate gears
		spin += 2;
	}

	////////////////////////////////////////////////////////////
	// 
	//  Main
	// 
	public static void main(String[] args) {
		javax.swing.JFrame frame = new javax.swing.JFrame("Involute Gears");
		frame.setSize(1200, 550);
		frame.getContentPane().add(new Gears());
		frame.setVisible(true);
	}
		
	///////////////////////////////////////////////////////////////////////////////////////
	//
	//  Utilities
	//

	public double rad(double degrees) {
		return degrees * PI / 180.0;
	}
	
	public double deg(double radians) {
		return radians * 180.0 / Math.PI;
	}

	public double ycord(double a, double r) {
		return cos(rad(a)) * r;
	}
	
	public double xcord(double a, double r) {
		return sin(rad(a)) * r;
	}

	public void drawRadius(java.awt.Graphics2D g, double angle, double radius) {
		double x = xcord(angle, radius);
		double y = ycord(angle, radius);
		g.draw(new Line2D.Double(xoffset, yoffset, x+xoffset, y+yoffset));
	}
	
	public void drawTangent(java.awt.Graphics2D g, double angle, double radius) {
		double x = xcord(angle, radius);
		double y = ycord(angle, radius);
		double tan_angle = angle + 90.0;
		double tan_len = step / 180.0 * PI * radius;
		double tx = xcord(tan_angle, tan_len);
		double ty = ycord(tan_angle, tan_len);
		tx += x;
		ty += y;
		g.draw(new Line2D.Double(x, y, tx, ty));
	}
	
	public void circle(java.awt.Graphics2D g, double radius) {
		GeneralPath p = new GeneralPath(GeneralPath.WIND_NON_ZERO);
		for(double a=0; a< 361; a+=3.0) {
			double angle = a / 180.0 * PI;
			double x = Math.cos(angle) * radius;
			double y = Math.sin(angle) * radius;
			if(a==0) {
				p.moveTo(x, y);
			} else {
				p.lineTo(x, y);
			}
		}
		p.closePath();
		g.draw(p);
	}
}
